use postgrest::Postgrest;
use reqwest::StatusCode;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::timebank::servicerating::service_rating_server::ServiceRating;
use crate::proto::timebank::servicerating::{create, delete, get, update, TServiceRating};
use crate::services::{error_messages, util, Result};

pub use crate::proto::timebank::servicerating::service_rating_server::ServiceRatingServer;

pub struct ServiceRatingService {
    db_client: Postgrest,
}

impl ServiceRatingService {
    pub fn new() -> Self {
        Self {
            db_client: util::miscellaneous::create_postgrest_client(),
        }
    }
}

#[tonic::async_trait]
impl ServiceRating for ServiceRatingService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let create::Payload {
                    user_id,
                    comment,
                    request_id,
                    value,
                } = payload;

                let res = self
                    .db_client
                    .rpc(
                        "rating_create",
                        json!({
                            "_user_id": user_id,
                            "_value": value,
                            "_comment": comment,
                            "_request_id": request_id
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values = res.json::<Vec<TServiceRating>>().await.unwrap();

                        Ok(Response::new(create::Response {
                            rating: values.into_iter().next(),
                        }))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::invalid_argument(error_messages::INVALID_PAYLOAD)),
        }
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let get::Payload { column, filter } = payload;
                let res = self
                    .db_client
                    .from("service_rating")
                    .eq(column, filter)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let ratings = res.json::<Vec<TServiceRating>>().await.unwrap();
                        Ok(Response::new(get::Response { ratings }))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let delete::Payload { rating_id } = payload;

                let res = self
                    .db_client
                    .rpc(
                        "rating_delete",
                        json!({ "_rating_id": rating_id }).to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::NO_CONTENT | StatusCode::OK => {
                        Ok(Response::new(delete::Response {}))
                    }
                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn update(
        &self,
        request: Request<update::Request>,
    ) -> Result<Response<update::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let update::Payload { rating_id, body } = payload;

                let res = self
                    .db_client
                    .from("service_rating")
                    .eq("id", rating_id)
                    .update(body)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values = res.json::<Vec<TServiceRating>>().await.unwrap();

                        Ok(Response::new(update::Response {
                            rating: values.into_iter().next(),
                        }))
                    }

                    _ => {
                        let mut s = Status::unknown(error_messages::UNKNOWN);
                        s.metadata_mut().append(
                            "error",
                            res.text().await.unwrap_or_default().parse().unwrap(),
                        );

                        Err(s)
                    }
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }
}
