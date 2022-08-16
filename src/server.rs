// TODO:
// 1. create auth middleware that checks for valid jwt
//    - how it'd work is by retrieving jwt attached in request metadata,
//      and get the user associated with the token. the append the user id
//      in the request metadata.
//

pub mod proto;
pub mod services;

use dotenv::dotenv;
use proto::{account::user_server::UserServer, auth::auth_server::AuthServer};
use services::collection::{
    service_rating::{ServiceRatingServer, ServiceRatingService},
    service_request::{ServiceRequestServer, ServiceRequestService},
    service_request_bid::{ServiceRequestBidServer, ServiceRequestBidService},
};
use services::{account::UserService, auth::AuthService};
use tonic::transport::Server;

// async fn interceptor(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
//     match req.metadata().get("access_token") {
//         Some(_) => Ok(req),

//         None => Err(tonic::Status::unauthenticated("MISSING ACCESS_TOKEN")),
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = dotenv::var("SOCKET_ADDRESS")
        .expect("MISSING SOCKET ADDRESS")
        .parse()
        .expect("UNABLE TO PARSE SOKCET ADDRESS STRING");

    Server::builder()
        .add_service(ServiceRequestServer::new(ServiceRequestService::new()))
        .add_service(ServiceRatingServer::new(ServiceRatingService::new()))
        .add_service(ServiceRequestBidServer::new(ServiceRequestBidService::new()))
        .add_service(UserServer::new(UserService::new()))
        .add_service(AuthServer::new(AuthService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
