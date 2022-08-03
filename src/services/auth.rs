///
/// Service for handling auth related operations eg login, regi&ster
///
// TODO:
// (1) Handle KYC during registration process
//
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
use crate::proto::auth::{login, register};
use crate::services::error_messages;
use reqwest::{self, StatusCode};

use crate::services::util::HTTP;

#[derive(Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<login::Request>,
    ) -> Result<Response<login::Response>, Status> {
        let payload = request.into_inner().payload;

        if let Some(payload) = payload {
            if payload.email.is_empty() || payload.password.is_empty() {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "Invalid Arguments",
                ));
            }

            let res = HTTP::post(
                "https://quepskrrpovzwydvfezs.supabase.co/auth/v1/token?grant_type=password",
            )
            .json(&payload)
            .send()
            .await
            .unwrap();

            let res_status = res.status();
            let res_data = res.json::<serde_json::Value>().await.unwrap();

            match res_status {
                StatusCode::OK => Ok(Response::new(login::Response {
                    auth_token: res_data["access_token"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    user_id: res_data["user"]["id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                })),

                StatusCode::BAD_REQUEST => Err(Status::new(
                    tonic::Code::InvalidArgument,
                    res_data["error_description"]
                        .as_str()
                        .unwrap_or_default()
                        .to_uppercase(),
                )),

                _ => Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN)),
            }
        } else {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            ))
        }
    }

    async fn register(
        &self,
        request: Request<register::Request>,
    ) -> Result<Response<register::Response>, Status> {
        let payload = request.into_inner().payload;

        if let Some(payload) = payload {
            let res = HTTP::post("https://quepskrrpovzwydvfezs.supabase.co/auth/v1/signup")
                .json(&json!({ "email": payload.email, "password": payload.password }))
                .send()
                .await
                .unwrap();

            match res.status() {
                StatusCode::OK => Ok(Response::new(register::Response {})),

                StatusCode::UNPROCESSABLE_ENTITY => Err(Status::new(
                    tonic::Code::ResourceExhausted,
                    error_messages::INVALID_PAYLOAD,
                )),

                StatusCode::TOO_MANY_REQUESTS => Err(Status::new(
                    tonic::Code::ResourceExhausted,
                    error_messages::TOO_MANY_REQUESTS,
                )),

                _ => Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN)),
            }
        } else {
            Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            ))
        }
    }
}
