///
/// Service for handling auth related operations eg sign_in, regi&ster
///
// TODO:
// (1) Handle KYC during registration process
//
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
use crate::proto::auth::{sign_in, sign_up};
use crate::services::error_messages;
use reqwest::{self, StatusCode};

use crate::services::util::HTTP;

#[derive(Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_in(
        &self,
        request: Request<sign_in::Request>,
    ) -> Result<Response<sign_in::Response>, Status> {
        let payload = request.into_inner().payload;

        if let Some(payload) = payload {
            if payload.email.is_empty() || payload.password.is_empty() {
                return Err(Status::invalid_argument("Missing email or password"));
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
                StatusCode::OK => Ok(Response::new(sign_in::Response {
                    auth_token: res_data["access_token"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    user_id: res_data["user"]["id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                })),

                StatusCode::BAD_REQUEST => Err(Status::invalid_argument(
                    res_data["error_description"]
                        .as_str()
                        .unwrap_or_default()
                        .to_uppercase(),
                )),

                _ => Err(Status::unknown(error_messages::UNKNOWN)),
            }
        } else {
            Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD))
        }
    }

    async fn sign_up(
        &self,
        request: Request<sign_up::Request>,
    ) -> Result<Response<sign_up::Response>, Status> {
        let payload = request.into_inner().payload;

        if let Some(payload) = payload {
            let res = HTTP::post("https://quepskrrpovzwydvfezs.supabase.co/auth/v1/signup")
                .json(&json!({ "email": payload.email, "password": payload.password }))
                .send()
                .await
                .unwrap();

            match res.status() {
                StatusCode::OK => Ok(Response::new(sign_up::Response {})),

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
