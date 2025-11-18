use tonic::{Request, Response, Status};

use crate::proto::{
    chat_service_server::ChatService,
    CreateChatRequest, CreateChatResponse,
    GetChatRequest, GetChatResponse,
    UpdateChatRequest, UpdateChatResponse,
    DeleteChatRequest, DeleteChatResponse,
    Chat, User,
};

#[derive(Debug, Default, Clone)]
pub struct ChatServiceImpl;

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn create_chat(
        &self,
        request: Request<CreateChatRequest>,
    ) -> Result<Response<CreateChatResponse>, Status> {
        Ok(Response::new(CreateChatResponse {
            chat: Some(Chat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![User {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                }],
                messages: vec![],
                created_at: "2025-01-01".to_string(),
                updated_at: "2025-01-01".to_string(),
                private: false,
            }),
        }))
    }

    async fn get_chat(
        &self,
        request: Request<GetChatRequest>,
    ) -> Result<Response<GetChatResponse>, Status> {
        Ok(Response::new(GetChatResponse {
            chat: Some(Chat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![User {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                }],
                messages: vec![],
                created_at: "2025-01-01".to_string(),
                updated_at: "2025-01-01".to_string(),
                private: false,
            }),
        }))
    }

    async fn update_chat(
        &self,
        request: Request<UpdateChatRequest>,
    ) -> Result<Response<UpdateChatResponse>, Status> {
        Ok(Response::new(UpdateChatResponse {
            chat: Some(Chat {
                id: "1".to_string(),
                name: "Test Chat".to_string(),
                description: "Test Description".to_string(),
                members: vec![User {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                }],
                messages: vec![],
                created_at: "2025-01-01".to_string(),
                updated_at: "2025-01-01".to_string(),
                private: false,
            }),
        }))
    }

    async fn delete_chat(
        &self,
        request: Request<DeleteChatRequest>,
    ) -> Result<Response<DeleteChatResponse>, Status> {
        Ok(Response::new(DeleteChatResponse {
            success: true,
        }))
    }
}