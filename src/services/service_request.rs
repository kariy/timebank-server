mod service_request {
    tonic::include_proto!("servicerequest");
}

use super::create_postgrest_client;
use postgrest::Postgrest;
use serde::Serialize;
use serde_json;
use service_request::request_server::Request as ServiceRequest;
use service_request::{create, get, BasicResponse};
use tonic::{Request, Response, Status};

pub use service_request::request_server::RequestServer;

const SERVICE_REQUEST_TABLE: &str = "service_request";

pub struct RequestService {
    client: Postgrest,
}

impl RequestService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RequestService {
            client: create_postgrest_client().unwrap(),
        })
    }
}

#[tonic::async_trait]
impl ServiceRequest for RequestService {
    async fn create(
        &self,
        request: Request<create::Request>,
    ) -> Result<Response<create::Response>, Status> {
        let details = request.into_inner().request.unwrap();

        #[derive(Serialize)]
        struct New {
            details: create::NewServiceRequest,
            requestor: String,
        }

        let new_request = New {
            details,
            requestor: "b3565b88-c0db-436d-ae66-5510577192f8".to_string(),
        };
        let new_request = serde_json::to_string(&new_request).unwrap();

        let res = self
            .client
            .from(SERVICE_REQUEST_TABLE)
            .insert(new_request)
            .execute()
            .await
            .unwrap();

        println!("{}", res.status());

        Ok(Response::new(create::Response {
            is_successful: if res.status() == 201 { true } else { false },
            request_id: None,
        }))
    }

    async fn delete(
        &self,
        request: Request<get::Request>,
    ) -> Result<Response<BasicResponse>, Status> {
        let request = request.into_inner();

        let res = self
            .client
            .from(SERVICE_REQUEST_TABLE)
            .eq("id", request.request_id)
            .delete()
            .execute()
            .await
            .unwrap();

        let is_successful = if res.status() == 204 { true } else { false };

        Ok(Response::new(BasicResponse { is_successful }))
    }

    async fn get(&self, request: Request<get::Request>) -> Result<Response<get::Response>, Status> {
        let request = request.into_inner();

        let res = self
            .client
            .from(SERVICE_REQUEST_TABLE)
            .eq("id", request.request_id)
            .select("*")
            .execute()
            .await
            .unwrap();

        println!("{}", res.text().await.unwrap());

        Ok(Response::new(get::Response {
            ..Default::default()
        }))
    }
}
