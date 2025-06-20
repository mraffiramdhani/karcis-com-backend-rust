use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

use crate::domain::models::StandardResponse;

#[derive(Debug, Display, Serialize)]
pub enum AppError {
    // Validation errors
    #[display(fmt = "Validation failed")]
    ValidationError(String),
    
    // Authentication errors
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    #[display(fmt = "Forbidden")]
    Forbidden,
    
    // Resource errors
    #[display(fmt = "Resource not found")]
    NotFound(String),
    #[display(fmt = "Resource already exists")]
    Conflict(String),
    
    // Database errors
    #[display(fmt = "Database error")]
    DatabaseError(String),
    
    // Internal errors
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(StandardResponse::<()>::error(
                    msg.clone(),
                    Some("VALIDATION_ERROR".to_string())
                ))
            },
            AppError::InvalidCredentials => {
                HttpResponse::Unauthorized().json(StandardResponse::<()>::error(
                    "Invalid credentials".to_string(),
                    Some("INVALID_CREDENTIALS".to_string())
                ))
            },
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(StandardResponse::<()>::error(
                    "Unauthorized access".to_string(),
                    Some("UNAUTHORIZED".to_string())
                ))
            },
            AppError::Forbidden => {
                HttpResponse::Forbidden().json(StandardResponse::<()>::error(
                    "Access forbidden".to_string(),
                    Some("FORBIDDEN".to_string())
                ))
            },
            AppError::NotFound(resource) => {
                HttpResponse::NotFound().json(StandardResponse::<()>::error(
                    format!("{} not found", resource),
                    Some("NOT_FOUND".to_string())
                ))
            },
            AppError::Conflict(resource) => {
                HttpResponse::Conflict().json(StandardResponse::<()>::error(
                    format!("{} already exists", resource),
                    Some("CONFLICT".to_string())
                ))
            },
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    "An internal error occurred".to_string(),
                    Some("INTERNAL_ERROR".to_string())
                ))
            },
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    "An internal error occurred".to_string(),
                    Some("INTERNAL_ERROR".to_string())
                ))
            },
        }
    }
}