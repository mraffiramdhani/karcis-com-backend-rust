use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl TokenPair {
    pub fn new(user_id: i64) -> Self {
        // Generate tokens here
        TokenPair {
            access_token: generate_access_token(user_id),
            refresh_token: generate_refresh_token(user_id),
        }
    }
}

fn generate_access_token(user_id: i64) -> String {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 3600; // Token expires in 1 hour

    let claims = json!({
        "sub": user_id.to_string(),
        "exp": expiration
    });

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )
    .unwrap_or_else(|_| String::new())
}

fn generate_refresh_token(user_id: i64) -> String {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 7 * 24 * 3600; // Token expires in 7 days

    let claims = json!({
        "sub": user_id.to_string(),
        "exp": expiration,
        "type": "refresh"
    });

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret("your_refresh_secret_key".as_ref()),
    )
    .unwrap_or_else(|_| String::new())
}

impl User {
    pub async fn find_by_username(
        pool: &sqlx::PgPool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await;
        result // Ensure the result is returned directly
    }

    pub async fn create(pool: &sqlx::PgPool, user: &RegisterUser) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&hash(&user.password, 10).unwrap())
        .fetch_one(pool)
        .await;
        result // Ensure the result is returned directly
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await;
        result // Ensure the result is returned directly
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}
