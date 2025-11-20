use crate::proto::{
    ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse, LogoutRequest,
    LogoutResponse, RegisterRequest, RegisterResponse, ResetPasswordRequest, ResetPasswordResponse,
    Session as GrpcSession, User as GrpcUser, Wallet as GrpcWallet, VerifyEmailRequest, VerifyEmailResponse,
    session_service_server::SessionService,
};

use tonic::{Request, Response, Status};
use time::OffsetDateTime;

#[derive(Debug, Default, Clone)]
pub struct SessionServiceImpl;

#[tonic::async_trait]
impl SessionService for SessionServiceImpl {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(RegisterResponse {
            user: Some(GrpcUser {
                id: "1".to_string(),
                display_name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                phone: None,
                experience_level: 0,
                experience_points: 0,
                experience_to_next_level: 0,
                coins: 0,
                wallet: Some(GrpcWallet {
                    id: "1".to_string(),
                    user_id: "1".to_string(),
                    coins: 0,
                    transactions: Vec::new(),
                }),
                mnstrs: Vec::new(),
            }),
        }))
    }

    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        Ok(Response::new(LoginResponse {
            session: Some(GrpcSession {
                id: "1".to_string(),
                user: Some(GrpcUser {
                    id: "1".to_string(),
                    display_name: "John Doe".to_string(),
                    email: "john.doe@example.com".to_string(),
                    phone: None,
                    experience_level: 0,
                    experience_points: 0,
                    experience_to_next_level: 0,
                    coins: 0,
                    wallet: Some(GrpcWallet {
                        id: "1".to_string(),
                        user_id: "1".to_string(),
                        coins: 0,
                        transactions: Vec::new(),
                    }),
                    mnstrs: Vec::new(),
                }),
                token: "1234567890".to_string(),
                expires_at: OffsetDateTime::now_utc().to_string(),
            }),
        }))
    }

    async fn logout(
        &self,
        _request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        Ok(Response::new(LogoutResponse { success: true }))
    }

    async fn forgot_password(
        &self,
        _request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        Ok(Response::new(ForgotPasswordResponse { success: true }))
    }

    async fn reset_password(
        &self,
        _request: Request<ResetPasswordRequest>,
    ) -> Result<Response<ResetPasswordResponse>, Status> {
        Ok(Response::new(ResetPasswordResponse { success: true }))
    }

    async fn verify_email(
        &self,
        _request: Request<VerifyEmailRequest>,
    ) -> Result<Response<VerifyEmailResponse>, Status> {
        Ok(Response::new(VerifyEmailResponse { success: true }))
    }
}
