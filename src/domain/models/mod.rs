pub mod auth;
pub mod otp;
pub mod token;
pub mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amenity {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub id: Option<i64>,
    pub user_id: i64,
    pub amount: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct StandardResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub timestamp: String,
}