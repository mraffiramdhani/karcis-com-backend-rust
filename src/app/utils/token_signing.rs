use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSigning {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i32,
    pub exp: usize,
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
