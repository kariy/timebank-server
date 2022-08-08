// Service for handling user's account

use postgrest::Postgrest;
use reqwest::StatusCode;
use tonic::{Request, Response, Status};

use crate::proto::account::user_server::User;
use crate::proto::account::{get_user, UserProfileData};
use crate::services::{error_messages, Result};

use super::util::create_postgrest_client;

pub struct UserService {
    db_client: Postgrest,
}

impl UserService {
    pub fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(UserService {
            db_client: create_postgrest_client().unwrap(),
        })
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<get_user::Request>,
    ) -> Result<Response<get_user::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) if !payload.user_id.is_empty() => {
                let res = self
                    .db_client
                    .from("user_profile")
                    .eq("user_id", payload.user_id)
                    .execute()
                    .await
                    .unwrap();

                let res_status = res.status();
                let res_data = res.json::<serde_json::Value>().await.unwrap();

                match res_status {
                    StatusCode::OK => {
                        let values =
                            serde_json::from_value::<Vec<UserProfileData>>(res_data).unwrap();

                        Ok(Response::new(get_user::Response {
                            user: values.into_iter().next(),
                        }))
                    }

                    StatusCode::BAD_REQUEST => {
                        Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
                    }

                    _ => Err(Status::unknown(error_messages::UNKNOWN)),
                }
            }
            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }
}
