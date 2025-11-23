use tonic::transport::Server;

use crate::proto::{
    chat_service_server::ChatServiceServer, session_service_server::SessionServiceServer,
};

pub mod proto {
    tonic::include_proto!("adventurers");
}

mod database;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let session_service =
        SessionServiceServer::new(services::sessions::SessionServiceImpl::default());
    let chat_service = ChatServiceServer::new(services::chats::ChatServiceImpl::default());
    println!("Server is running on {}", addr);
    Server::builder()
        .add_service(session_service)
        .add_service(chat_service)
        .serve(addr)
        .await?;
    Ok(())
}
