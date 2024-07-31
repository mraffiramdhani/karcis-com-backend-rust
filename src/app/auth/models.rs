use bcrypt::{hash, verify};
use chrono::{DateTime, NaiveDateTime, Utc};
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
    pub title: String,
    pub image: String,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i64,
    pub token: String,
    pub is_revoked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn find_by_username_or_email(
        pool: &sqlx::PgPool,
        username: &str,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result =
            sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username = $1 OR email = $2")
                .bind(username)
                .bind(email)
                .fetch_optional(pool)
                .await;
        result // Ensure the result is returned directly
    }

    pub async fn create(pool: &sqlx::PgPool, user: &RegisterUser) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO users (first_name, last_name, phone, username, email, password_hash, title, image) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&hash(&user.password, 10).unwrap())
        .bind(&user.title)
        .bind(&user.image)
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

impl Token {
    pub async fn create(pool: &sqlx::PgPool, token: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>("INSERT INTO revoked_token (token) VALUES ($1) RETURNING *")
            .bind(token)
            .fetch_one(pool)
            .await
    }

    pub async fn is_token_revoked(pool: &sqlx::PgPool, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_token WHERE token = $1 AND is_revoked = true)",
        )
        .bind(token)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn revoke_token(pool: &sqlx::PgPool, token: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>("UPDATE revoked_token SET is_revoked = true WHERE token = $1")
            .bind(token)
            .fetch_one(pool)
            .await
    }
}
