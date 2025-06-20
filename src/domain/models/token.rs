use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i64,
    pub token: String,
    pub is_revoked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSigning {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i32,
    pub exp: usize,
}
