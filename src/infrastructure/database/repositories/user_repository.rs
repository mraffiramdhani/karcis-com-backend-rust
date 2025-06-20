use super::{Repository, TransactionalRepository};
use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::infrastructure::database::PostgresPool;

pub struct UserRepository {
    pool: PostgresPool,
}

impl UserRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, username, email, password_hash, 
                   title, image, phone, role_id, deleted_at
            FROM users
            WHERE username = $1 AND deleted_at IS NULL
            "#,
            username
        )
        .fetch_optional(self.pool.pool())
        .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, username, email, password_hash, 
                   title, image, phone, role_id, deleted_at
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
            email
        )
        .fetch_optional(self.pool.pool())
        .await
    }
}

#[async_trait]
impl Repository<User, i64> for UserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, username, email, password_hash, 
                   title, image, phone, role_id, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(self.pool.pool())
        .await
    }

    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, username, email, password_hash, 
                   title, image, phone, role_id, deleted_at
            FROM users
            WHERE deleted_at IS NULL
            "#
        )
        .fetch_all(self.pool.pool())
        .await
    }

    async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin_transaction().await?;
        let result = self.create_tx(&mut tx, user).await?;
        tx.commit().await?;
        Ok(result)
    }

    async fn update(&self, user: &User) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin_transaction().await?;
        let result = self.update_tx(&mut tx, user).await?;
        tx.commit().await?;
        Ok(result)
    }

    async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let mut tx = self.pool.begin_transaction().await?;
        let result = self.delete_tx(&mut tx, id).await?;
        tx.commit().await?;
        Ok(result)
    }
}

#[async_trait]
impl TransactionalRepository<User, i64> for UserRepository {
    async fn find_by_id_tx(&self, tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, username, email, password_hash, 
                   title, image, phone, role_id, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&mut **tx)
        .await
    }

    async fn create_tx(&self, tx: &mut Transaction<'_, Postgres>, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (first_name, last_name, username, email, password_hash, 
                             title, image, phone, role_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, first_name, last_name, username, email, password_hash, 
                      title, image, phone, role_id, deleted_at
            "#,
            user.first_name,
            user.last_name,
            user.username,
            user.email,
            user.password_hash,
            user.title,
            user.image,
            user.phone,
            user.role_id
        )
        .fetch_one(&mut **tx)
        .await
    }

    async fn update_tx(&self, tx: &mut Transaction<'_, Postgres>, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name = $1,
                last_name = $2,
                username = $3,
                email = $4,
                password_hash = $5,
                title = $6,
                image = $7,
                phone = $8,
                role_id = $9
            WHERE id = $10 AND deleted_at IS NULL
            RETURNING id, first_name, last_name, username, email, password_hash, 
                      title, image, phone, role_id, deleted_at
            "#,
            user.first_name,
            user.last_name,
            user.username,
            user.email,
            user.password_hash,
            user.title,
            user.image,
            user.phone,
            user.role_id,
            user.id
        )
        .fetch_one(&mut **tx)
        .await
    }

    async fn delete_tx(&self, tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected() > 0)
    }
} 