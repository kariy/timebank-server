use super::create_postgrest_client;
use postgrest::Postgrest;
use serde::Serialize;
use serde_json;
use tonic::{Request, Response, Status};

pub mod service_request {
    tonic::include_proto!("servicerequest");
}

pub mod generic {
    tonic::include_proto!("generic");
}

use service_request::service_request_server::ServiceRequest;
use service_request::{create, create::CreateServiceRequestPayload};

const SERVICE_REQUEST_TABLE: &str = "service_request";

pub struct ServiceRequestService {
    client: Postgrest,
}

impl ServiceRequestService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ServiceRequestService {
            client: create_postgrest_client().unwrap(),
        })
    }
}

#[tonic::async_trait]
impl ServiceRequest for ServiceRequestService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<generic::Response>, Status> {
        let request = request.into_inner();

        let (is_successful, response_data) = match request.payload {
            Some(CreateServiceRequestPayload {
                request: Some(new_request_data),
                // user_id
            }) => {
                let res = self
                    .client
                    .from(SERVICE_REQUEST_TABLE)
                    .insert(serde_json::to_string(&new_request_data).unwrap())
                    .execute()
                    .await
                    .unwrap();

                let status = res.status().as_u16();

                let is_successful = if status >= 200 && status < 300 {
                    true
                } else {
                    false
                };

                let response_data = res.text().await.unwrap();

                (is_successful, response_data)
            }
            _ => {
                // send 'invalid request payload format' response

                let error = generic::Error {
                    details: "".to_string(),
                    message: "invalid request payload format".to_string(),
                };

                (false, serde_json::to_string(&error).unwrap())
            }
        };

        Ok(Response::new(generic::Response {
            is_successful,
            payload: response_data,
        }))

        // let (new_request_data, requestor) = (data.request.unwrap(), data.user_id);

        // let new_request_str = {
        //     let new = NewServiceRequestData {
        //         details: new_request_data.details.unwrap(),
        //         rate: new_request_data.rate,
        //         requestor: requestor,
        //     };

        //     serde_json::to_string(&new).unwrap()
        // };

        // let (is_successful, create_response_oneof) = {
        //     let res = self
        //         .client
        //         .from(SERVICE_REQUEST_TABLE)
        //         .insert(new_request_str)
        //         .execute()
        //         .await
        //         .unwrap();
        // };

        // Ok(Response::new(create::Response {
        //     is_successful,
        //     create_response_oneof,
        // }))
    }

    // async fn delete(
    //     &self,
    //     request: Request<get_one::Request>,
    // ) -> Result<Response<BasicResponse>, Status> {
    //     let request = request.into_inner();

    //     let res = self
    //         .client
    //         .from(SERVICE_REQUEST_TABLE)
    //         .eq("id", request.request_id)
    //         .delete()
    //         .execute()
    //         .await
    //         .unwrap();

    //     let is_successful = if res.status() == 204 { true } else { false };

    //     Ok(Response::new(BasicResponse { is_successful }))
    // }

    // async fn get_one(
    //     &self,
    //     request: Request<get_one::Request>,
    // ) -> Result<Response<get_one::Response>, Status> {
    //     let request = request.into_inner();

    //     let res = self
    //         .client
    //         .from(SERVICE_REQUEST_TABLE)
    //         .eq("id", request.request_id)
    //         .select("*,service_request_bid(*)")
    //         .execute()
    //         .await
    //         .unwrap();

    //     // println!("{:#}", res.text().await.unwrap());

    //     Ok(Response::new(get_one::Response {
    //         request: match res.json::<Vec<service_request::ServiceRequest>>().await {
    //             Ok(data) => Some(data[0].clone()),
    //             Err(_) => None,
    //         },
    //     }))
    // }
}
