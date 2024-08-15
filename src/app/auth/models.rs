use bcrypt::{hash, verify};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, PgPool, Postgres, Transaction};

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
    pub role_id: i32,
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
pub struct ForgotPassword {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckOTP {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPassword {
    pub email: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub title: String,
    pub image: String,
    pub role_id: i32,
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
    pub async fn find_by(
        pool: &PgPool,
        field: &str,
        value: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let query = format!("SELECT * FROM users WHERE {} = $1", field);
        let result = sqlx::query_as::<_, Self>(&query)
            .bind(value)
            .fetch_optional(pool)
            .await;
        result
    }

    pub async fn find_by_username_or_email(
        pool: &PgPool,
        username: &str,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result =
            sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username = $1 OR email = $2")
                .bind(username)
                .bind(email)
                .fetch_optional(pool)
                .await;
        result
    }

    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await;
        result
    }

    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        user: &RegisterUser,
    ) -> Result<Self, sqlx::Error> {
        let password_hash = hash(&user.password, 10).unwrap();
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO users (first_name, last_name, phone, username, email, password_hash, title, image) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(&user.title)
        .bind(&user.image)
        .fetch_one(&mut **transaction)
        .await;
        result
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await;
        result
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    pub async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        user: &Profile,
    ) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>(
            "UPDATE users SET first_name = $1, last_name = $2, phone = $3, username = $4, email = $5, title = $6, image = $7 WHERE id = $8 RETURNING *",
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.title)
        .bind(&user.image)
        .bind(&user.id)
        .fetch_one(&mut **transaction)
        .await;
        result
    }

    pub async fn update_password(
        transaction: &mut Transaction<'_, Postgres>,
        user: &ResetPassword,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let password_hash = hash(&user.new_password, 10).unwrap();
        let result = sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2 RETURNING *")
            .bind(&password_hash)
            .bind(&user.email)
            .execute(&mut **transaction)
            .await;
        result
    }
}

impl Token {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        token: &str,
    ) -> Result<Self, sqlx::Error> {
        let result =
            sqlx::query_as::<_, Self>("INSERT INTO revoked_token (token) VALUES ($1) RETURNING *")
                .bind(token)
                .fetch_one(&mut **transaction)
                .await;
        result
    }

    pub async fn is_token_revoked(
        transaction: &mut Transaction<'_, Postgres>,
        token: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_token WHERE token = $1 AND is_revoked = true)",
        )
        .bind(token)
        .fetch_one(&mut **transaction)
        .await?;
        Ok(result)
    }

    pub async fn revoke_token(
        transaction: &mut Transaction<'_, Postgres>,
        token: String,
    ) -> Result<u64, sqlx::Error> {
        match sqlx::query("UPDATE revoked_token SET is_revoked = true WHERE token = $1")
            .bind(&token)
            .execute(&mut **transaction)
            .await
        {
            Ok(row) => Ok(row.rows_affected()),
            Err(e) => Err(e),
        }
    }
}
