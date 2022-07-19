mod services;

use dotenv::dotenv;
use services::service_request::{RequestServer, RequestService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "[::1]:50051".parse()?;

    let request_service = RequestService::new().unwrap();

    Server::builder()
        .add_service(RequestServer::new(request_service))
        .serve(addr)
        .await?;

    Ok(())
}
