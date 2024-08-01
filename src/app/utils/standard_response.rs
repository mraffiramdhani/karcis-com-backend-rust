use serde::Serialize;

#[derive(Serialize)]
pub struct StandardResponse<T> {
    success: bool,
    message: String,
    data: Option<T>, // Changed to Option<T>
}

impl<T: Serialize> StandardResponse<T> {
    pub fn ok(data: T, message: Option<String>) -> Self {
        StandardResponse {
            success: true,
            message: if let Some(m) = message {
                m
            } else {
                "Operation successful".to_string()
            },
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        StandardResponse {
            success: false,
            message,
            data: None,
        }
    }
}
