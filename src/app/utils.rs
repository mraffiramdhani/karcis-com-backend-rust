use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSigning {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl TokenSigning {
    pub fn sign(data: Self) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = dotenv::var("APP_KEY").unwrap();
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        jsonwebtoken::encode(
            &header,
            &data,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn verify(
        token: &str,
    ) -> Result<jsonwebtoken::TokenData<Self>, jsonwebtoken::errors::Error> {
        let secret = dotenv::var("APP_KEY").unwrap();
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        jsonwebtoken::decode::<Self>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
    }
}
