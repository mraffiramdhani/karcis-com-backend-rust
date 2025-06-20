use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub title: String,
    pub image: String,
    pub phone: String,
    pub role_id: i32,
    pub deleted_at: Option<DateTime<Utc>>,
}
