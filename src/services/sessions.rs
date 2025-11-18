use crate::proto::{
    session_service_server::SessionService, LoginRequest, LoginResponse, LogoutRequest, LogoutResponse,
    RegisterRequest, RegisterResponse, Session, User,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default, Clone)]
pub struct SessionServiceImpl;

#[tonic::async_trait]
impl SessionService for SessionServiceImpl {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(RegisterResponse {
            user: Some(User {
                id: "1".to_string(),
                display_name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
            }),
        }))
    }

    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        Ok(Response::new(LoginResponse {
            session: Some(Session {
                id: "1".to_string(),
                user: Some(User {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                }),
                token: "1234567890".to_string(),
                expires_at: "2025-01-01".to_string(),
            }),
        }))
    }

    async fn logout(
        &self,
        _request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        Ok(Response::new(LogoutResponse { success: true }))
    }
}
