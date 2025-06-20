use serde::Serialize;
use crate::domain::models::StandardResponse;

impl<T: Serialize> StandardResponse<T> {
    pub fn ok(data: T, message: Option<String>) -> Self {
        StandardResponse {
            success: true,
            message: message.unwrap_or_else(|| "Operation successful".to_string()),
            data: Some(data),
            error_code: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: String, error_code: Option<String>) -> Self {
        StandardResponse {
            success: false,
            message,
            data: None,
            error_code,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}