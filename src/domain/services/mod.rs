pub mod auth;
pub mod otp;
pub mod token;
pub mod user;

use crate::domain::models::{
    auth::{ForgotPasswordPayload, LoginPayload, RegisterPayload},
    token::Token,
    user::User,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Send {
    async fn find_by(&self, field: &str, value: &str) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, user: &RegisterPayload) -> Result<(User, String), sqlx::Error>;
    async fn login(&self, data: &LoginPayload) -> Result<(User, String), sqlx::Error>;
}

#[async_trait]
pub trait TokenService {
    async fn create(&self, token: &str) -> Result<Token, sqlx::Error>;
    async fn is_token_revoked(&self, token: &str) -> Result<bool, sqlx::Error>;
    async fn revoke_token(&self, token: &str) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait AuthService {
    async fn forgot_password(&self, data: &ForgotPasswordPayload) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait OtpService: Send {
    async fn create(&self, otp: &str) -> Result<(), sqlx::Error>;
}