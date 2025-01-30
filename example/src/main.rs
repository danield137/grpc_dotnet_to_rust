use std::net::SocketAddr;

use prost::Message;
mod generated;
use generated::user::{self, users_ctrl_server::{UsersCtrl, UsersCtrlServer}, User};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UsersService {}

#[tonic::async_trait]
impl UsersCtrl for UsersService {
    async fn post_user(
        &self,
        request: Request<User>,
    ) -> Result<Response<User>, Status> {
        let name = request.into_inner().name;
        println!("Received request: {}", name.unwrap()); // Print received message

        let reply = User::default();
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let users_svc = UsersService::default();

    println!("Starting gRPC server at {}", addr);

    Server::builder()
        .add_service(UsersCtrlServer::new(users_svc))
        .serve(addr)
        .await?;

    Ok(())
}
