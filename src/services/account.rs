// Service for handling user's account

use postgrest::Postgrest;
use reqwest::StatusCode;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::account::user_server::User;
use crate::proto::account::{get, get_rating, update, TUserProfile};
use crate::proto::timebank::servicerating::TServiceRating;
use crate::services::{error_messages, util, Result};

pub struct UserService {
    db_client: Postgrest,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            db_client: util::miscellaneous::create_postgrest_client(),
        }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
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

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TUserProfile> = res.json().await.unwrap();

                        Ok(Response::new(get::Response {
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

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) if !payload.user_id.is_empty() => {
                let update::Payload { update, user_id } = payload;

                let res = self
                    .db_client
                    .from("user_profile")
                    .eq("user_id", user_id)
                    .update(update)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TUserProfile> = res.json().await.unwrap();

                        Ok(Response::new(update::Response {
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

    async fn get_rating(
        &self,
        request: Request<get_rating::Request>,
    ) -> Result<Response<get_rating::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .rpc(
                        "user_get_rating",
                        json!({
                            "_user_id": payload.user_id,
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let ratings: Vec<TServiceRating> = res
                            .json()
                            .await
                            .expect("UNABLE TO PARSE RESPONSE DATA AS `Vec<TServiceRating>`");

                        Ok(Response::new(get_rating::Response { ratings }))
                    }

                    _ => Err(Status::unknown(error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }
}
