use tonic::{transport::Server};

use crate::proto::{
    session_service_server::SessionServiceServer,
    chat_service_server::ChatServiceServer,
};

pub mod proto {
    tonic::include_proto!("adventurers");
}

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8000".parse().unwrap();
    let session_service = SessionServiceServer::new(services::sessions::SessionServiceImpl::default());
    let chat_service = ChatServiceServer::new(services::chats::ChatServiceImpl::default());
    println!("Server is running on {}", addr);
    Server::builder()
        .add_service(session_service)
        .add_service(chat_service)
        .serve(addr)
        .await?;
    Ok(())
}
