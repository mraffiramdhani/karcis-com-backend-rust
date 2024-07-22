use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum AuthError {
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "User not found")]
    UserNotFound,
    #[display(fmt = "Username already exists")]
    UsernameAlreadyExists,
    #[display(fmt = "Email already exists")]
    EmailAlreadyExists,
    #[display(fmt = "Internal server error")]
    InternalServerError,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials => HttpResponse::Unauthorized().json(self),
            AuthError::UserNotFound => HttpResponse::NotFound().json(self),
            AuthError::UsernameAlreadyExists => HttpResponse::BadRequest().json(self),
            AuthError::EmailAlreadyExists => HttpResponse::BadRequest().json(self),
            AuthError::InternalServerError => HttpResponse::InternalServerError().json(self),
            AuthError::Unauthorized => HttpResponse::Unauthorized().json(self),
        }
    }
}
