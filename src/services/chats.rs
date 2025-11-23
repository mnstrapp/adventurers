use tonic::{Request, Response, Status};

use crate::{
    proto::{
        Chat as GrpcChat, CreateChatRequest, CreateChatResponse, DeleteChatRequest,
        DeleteChatResponse, GetChatRequest, GetChatResponse, UpdateChatRequest, UpdateChatResponse,
        User as GrpcUser, chat_service_server::ChatService,
    },
    utils::time::to_prost_timestamp,
};

use time::OffsetDateTime;

#[derive(Debug, Default, Clone)]
pub struct ChatServiceImpl;

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn create_chat(
        &self,
        request: Request<CreateChatRequest>,
    ) -> Result<Response<CreateChatResponse>, Status> {
        Ok(Response::new(CreateChatResponse {
            chat: Some(GrpcChat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![GrpcUser {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: None,
                    experience_level: 0,
                    experience_points: 0,
                    experience_to_next_level: 0,
                    coins: 0,
                    wallet: None,
                    mnstrs: Vec::new(),
                }],
                messages: vec![],
                created_at: OffsetDateTime::now_utc().to_string().into(),
                updated_at: OffsetDateTime::now_utc().to_string().into(),
                private: false,
            }),
        }))
    }

    async fn get_chat(
        &self,
        request: Request<GetChatRequest>,
    ) -> Result<Response<GetChatResponse>, Status> {
        Ok(Response::new(GetChatResponse {
            chat: Some(GrpcChat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![GrpcUser {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: None,
                    experience_level: 0,
                    experience_points: 0,
                    experience_to_next_level: 0,
                    coins: 0,
                    wallet: None,
                    mnstrs: Vec::new(),
                }],
                messages: vec![],
                created_at: OffsetDateTime::now_utc().to_string().into(),
                updated_at: OffsetDateTime::now_utc().to_string().into(),
                private: false,
            }),
        }))
    }

    async fn update_chat(
        &self,
        request: Request<UpdateChatRequest>,
    ) -> Result<Response<UpdateChatResponse>, Status> {
        Ok(Response::new(UpdateChatResponse {
            chat: Some(GrpcChat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![GrpcUser {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: None,
                    experience_level: 0,
                    experience_points: 0,
                    experience_to_next_level: 0,
                    coins: 0,
                    wallet: None,
                    mnstrs: Vec::new(),
                }],
                messages: vec![],
                created_at: OffsetDateTime::now_utc().to_string().into(),
                updated_at: OffsetDateTime::now_utc().to_string().into(),
                private: false,
            }),
        }))
    }

    async fn delete_chat(
        &self,
        request: Request<DeleteChatRequest>,
    ) -> Result<Response<DeleteChatResponse>, Status> {
        Ok(Response::new(DeleteChatResponse { success: true }))
    }
}
