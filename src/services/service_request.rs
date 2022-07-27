// TODO:
// 1. make sure methods' success/error responses are consistent

use crate::services::{util::create_postgrest_client, Result};
use postgrest::Postgrest;
use reqwest::StatusCode;
use tonic::{Request, Response, Status};

use crate::proto::service_request::service_request_server::ServiceRequest;
use crate::proto::service_request::{create, delete, delete_bid, get_by_id, place_bid};
use crate::proto::service_request::{ServiceRequestBid, ServiceRequestData};
use crate::services::error_messages;

const SERVICE_REQUEST_TABLE: &str = "service_request";
const SERVICE_REQUEST_BID_TABLE: &str = "service_request_bid";

pub struct ServiceRequestService {
    client: Postgrest,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Default, Debug)]
struct DatabaseError {
    message: String,
    code: String,
    details: String,
    hint: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Default, Debug)]
struct DatabaseErrorResponse {
    response_status: u16,
    error: DatabaseError,
}

impl ServiceRequestService {
    pub fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(ServiceRequestService {
            client: create_postgrest_client().unwrap(),
        })
    }

    /// Returns an array of Service Request based on the provided `filter`
    async fn get<T>(
        &self,
        column: T,
        filter: T,
    ) -> std::result::Result<Vec<ServiceRequestData>, DatabaseErrorResponse>
    where
        T: AsRef<str>,
    {
        let res = self
            .client
            .from(SERVICE_REQUEST_TABLE)
            .select("*")
            .eq(column, filter)
            .execute()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                Ok(serde_json::from_str(&res.text().await.unwrap()).unwrap_or_default())
            }

            _ => Err(DatabaseErrorResponse {
                response_status: res.status().as_u16(),
                error: serde_json::from_str(&res.text().await.unwrap()).unwrap_or_default(),
            }),
        }
    }

    // async fn is_authorize(&self) {}
}

#[tonic::async_trait]
impl ServiceRequest for ServiceRequestService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let body = serde_json::to_string(&payload).unwrap();

                let res = self
                    .client
                    .from(SERVICE_REQUEST_TABLE)
                    .insert(body)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::CREATED => {
                        let values: Vec<ServiceRequestData> =
                            serde_json::from_str(&res.text().await.unwrap()).unwrap_or_default();

                        Ok(Response::new(create::Response {
                            request: values.into_iter().next(),
                        }))
                    }

                    StatusCode::BAD_REQUEST => Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "Missing Expected Data",
                    )),

                    StatusCode::CONFLICT => Err(Status::new(
                        tonic::Code::FailedPrecondition,
                        "Data Conflict",
                    )),

                    _ => Err(Status::new(tonic::Code::Internal, error_messages::UNKNOWN)),
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
                {
                    let values = self
                        .get("id", &payload.request_id)
                        .await
                        .unwrap_or_default();

                    if values.is_empty() {
                        return Err(Status::new(
                            tonic::Code::FailedPrecondition,
                            "ITEM DOESN'T EXIST",
                        ));
                    }
                }

                let res = self
                    .client
                    .from(SERVICE_REQUEST_TABLE)
                    .eq("id", payload.request_id)
                    .delete()
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => Ok(Response::new(delete::Response {})),

                    _ => Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<get_by_id::Request>,
    ) -> Result<Response<get_by_id::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                let values = self
                    .get("id", &payload.request_id)
                    .await
                    .unwrap_or_default();

                Ok(Response::new(get_by_id::Response {
                    request: values.into_iter().next(),
                }))
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn place_bid(
        &self,
        request: Request<place_bid::Request>,
    ) -> Result<Response<place_bid::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                // Check if user already bid on`request_id`
                {
                    let res = self
                        .client
                        .from(SERVICE_REQUEST_BID_TABLE)
                        .eq("request_id", &payload.request_id)
                        .eq("bidder", &payload.bidder)
                        .execute()
                        .await
                        .unwrap();

                    let data =
                        serde_json::from_str::<Vec<ServiceRequestBid>>(&res.text().await.unwrap());

                    match data {
                        Ok(data) => {
                            if !data.is_empty() {
                                return Err(Status::new(
                                    tonic::Code::AlreadyExists,
                                    error_messages::ALREADY_EXISTS,
                                ));
                            }
                        }

                        Err(_) => {
                            return Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN))
                        }
                    }
                }

                let body = serde_json::to_string(&payload).unwrap();

                let res = self
                    .client
                    .from(SERVICE_REQUEST_BID_TABLE)
                    .insert(body)
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::CREATED => {
                        let bids: Vec<ServiceRequestBid> =
                            serde_json::from_str(&res.text().await.unwrap()).unwrap_or_default();

                        Ok(Response::new(place_bid::Response {
                            bid: bids.into_iter().next(),
                        }))
                    }

                    StatusCode::BAD_REQUEST => Err(Status::new(
                        tonic::Code::InvalidArgument,
                        error_messages::INVALID_PAYLOAD,
                    )),

                    _ => Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }

    async fn delete_bid(
        &self,
        request: Request<delete_bid::Request>,
    ) -> Result<Response<delete_bid::Response>> {
        let payload = request.into_inner().payload;

        match payload {
            Some(payload) => {
                {
                    let res = self
                        .client
                        .from(SERVICE_REQUEST_BID_TABLE)
                        .eq("id", &payload.bid_id)
                        .execute()
                        .await
                        .unwrap();

                    let data =
                        serde_json::from_str::<Vec<ServiceRequestBid>>(&res.text().await.unwrap())
                            .unwrap_or_default();

                    if data.is_empty() {
                        return Err(Status::new(
                            tonic::Code::FailedPrecondition,
                            "ITEM DOESN'T EXIST",
                        ));
                    }
                }

                let res = self
                    .client
                    .from(SERVICE_REQUEST_BID_TABLE)
                    .eq("id", &payload.bid_id)
                    .delete()
                    .execute()
                    .await
                    .unwrap();

                match res.status() {
                    StatusCode::OK => Ok(Response::new(delete_bid::Response {})),

                    _ => Err(Status::new(tonic::Code::Unknown, error_messages::UNKNOWN)),
                }
            }

            _ => Err(Status::new(
                tonic::Code::InvalidArgument,
                error_messages::INVALID_PAYLOAD,
            )),
        }
    }
}
