use actix_web::HttpResponse;
use crate::domain::models::StandardResponse;

pub fn handle_database_error<T>(error: sqlx::Error, operation: &str) -> HttpResponse {
    log::error!("Database error during {}: {:?}", operation, error);
    
    match error {
        sqlx::Error::RowNotFound => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error(
                "Resource not found".to_string(),
                Some("NOT_FOUND".to_string())
            ))
        },
        sqlx::Error::Database(db_error) => {
            if let Some(code) = db_error.code() {
                match code.as_ref() {
                    "23505" => { // Unique violation
                        HttpResponse::Conflict().json(StandardResponse::<()>::error(
                            "Resource already exists".to_string(),
                            Some("CONFLICT".to_string())
                        ))
                    },
                    _ => {
                        HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                            "An internal error occurred".to_string(),
                            Some("INTERNAL_ERROR".to_string())
                        ))
                    }
                }
            } else {
                HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    "An internal error occurred".to_string(),
                    Some("INTERNAL_ERROR".to_string())
                ))
            }
        },
        _ => {
            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                "An internal error occurred".to_string(),
                Some("INTERNAL_ERROR".to_string())
            ))
        }
    }
}

pub fn handle_validation_error(errors: Vec<String>) -> HttpResponse {
    HttpResponse::BadRequest().json(StandardResponse::<()>::error(
        format!("Validation failed: {}", errors.join(", ")),
        Some("VALIDATION_ERROR".to_string())
    ))
}

pub fn handle_invalid_credentials() -> HttpResponse {
    HttpResponse::Unauthorized().json(StandardResponse::<()>::error(
        "Invalid credentials".to_string(),
        Some("INVALID_CREDENTIALS".to_string())
    ))
}

pub fn handle_error<T>(error: Box<dyn std::error::Error>, operation: &str) -> HttpResponse {
    log::error!("Error during {}: {:?}", operation, error);
    HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
        "An internal error occurred".to_string(),
        Some("INTERNAL_ERROR".to_string())
    ))
}