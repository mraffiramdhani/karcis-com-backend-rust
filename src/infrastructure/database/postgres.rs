use sqlx::postgres::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresPool {
    pool: Arc<PgPool>,
}

impl PostgresPool {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
    //     sqlx::query("SELECT 1")
    //         .execute(self.pool())
    //         .await?;
    //     Ok(())
    // }

    pub async fn begin_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error> {
        self.pool().begin().await
    }
} 