use postgrest::Postgrest;
use reqwest::StatusCode;
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::proto::timebank::servicerequestbid::service_request_bid_server::ServiceRequestBid;
use crate::proto::timebank::servicerequestbid::{create, delete, get, TServiceRequestBid};
use crate::services::{error_messages, util, Result};

pub use crate::proto::timebank::servicerequestbid::service_request_bid_server::ServiceRequestBidServer;

pub struct ServiceRequestBidService {
    db_client: Postgrest,
}

impl ServiceRequestBidService {
    pub fn new() -> Self {
        Self {
            db_client: util::miscellaneous::create_postgrest_client(),
        }
    }
}

// async fn handle_db_response<F, T>(
//     res: reqwest::Response,
//     f: F,
// ) -> std::result::Result<Response<T>, Status>
// where
//     F: FnOnce(
//         reqwest::Response,
//     ) -> core::pin::Pin<
//         Box<dyn ::core::future::Future<Output = Result<Response<T>>> + ::core::marker::Send>,
//     >,
// {
//     match res.status() {
//         StatusCode::OK => f(res).await,

//         _ => {
//             let mut s = Status::unknown(error_messages::UNKNOWN);
//             s.metadata_mut().append(
//                 "error",
//                 res.text().await.unwrap_or_default().parse().unwrap(),
//             );

//             Err(s)
//         }
//     }
// }

#[tonic::async_trait]
impl ServiceRequestBid for ServiceRequestBidService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .rpc(
                        "bid_create",
                        json!({
                            "_user_id": payload.user_id,
                            "_request_id": payload.request_id,
                            "_amount": payload.amount
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => {
                        let values: Vec<TServiceRequestBid> = res.json().await.unwrap();

                        Ok(Response::new(create::Response {
                            bid: values.into_iter().next(),
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

    async fn delete(
        &self,
        request: Request<delete::Request>,
    ) -> Result<Response<delete::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let res = self
                    .db_client
                    .rpc(
                        "bid_delete",
                        json!({
                            "_bid_id": payload.bid_id
                        })
                        .to_string(),
                    )
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => Ok(Response::new(delete::Response {})),

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

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let get::Payload { column, filter } = payload;
                let res = self
                    .db_client
                    .from("service_request_bid")
                    .eq(column, filter)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => Ok(Response::new(get::Response {
                        bids: res.json::<Vec<TServiceRequestBid>>().await.unwrap(),
                    })),

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
