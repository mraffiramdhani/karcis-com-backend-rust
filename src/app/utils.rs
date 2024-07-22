use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct StandardResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> StandardResponse<T> {
    pub fn new(success: bool, message: String, data: Option<T>) -> Self {
        StandardResponse {
            success,
            message,
            data,
        }
    }

    pub fn ok(data: T) -> Self {
        StandardResponse {
            success: true,
            message: "Operation successful".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> StandardResponse<()> {
        StandardResponse {
            success: false,
            message,
            data: None,
        }
    }

    pub fn to_json_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
