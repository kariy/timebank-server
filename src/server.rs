pub mod proto;
pub mod services;

use dotenv::dotenv;
use tonic::{transport::Server, Request, Status};

use proto::auth::auth_server::AuthServer;
use proto::service_request::service_request_server::ServiceRequestServer;
use services::auth::AuthService;
use services::service_request::ServiceRequestService;

fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    println!("middleware {:?}", req.metadata());
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "[::1]:50051".parse()?;

    let request_service = ServiceRequestService::new().unwrap();
    let auth_service = AuthService::default();

    Server::builder()
        .add_service(ServiceRequestServer::with_interceptor(
            request_service,
            interceptor,
        ))
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
