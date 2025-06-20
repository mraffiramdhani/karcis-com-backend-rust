use crate::{
    domain::models::token::Token, domain::services::TokenService,
    infrastructure::database::PostgresPool,
};
use async_trait::async_trait;

#[async_trait]
impl TokenService for PostgresPool {
    async fn create(&self, token: &str) -> Result<Token, sqlx::Error> {
        let token =
            sqlx::query_as::<_, Token>("INSERT INTO revoked_token (token) VALUES ($1) RETURNING *")
                .bind(token)
                .fetch_one(self.pool())
                .await?;
        Ok(token)
    }
    async fn is_token_revoked(&self, token: &str) -> Result<bool, sqlx::Error> {
        let token = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_token WHERE token = $1 AND is_revoked = true)",
        )
        .bind(token)
        .fetch_one(self.pool())
        .await?;
        Ok(token)
    }
    async fn revoke_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE revoked_token SET is_revoked = true WHERE token = $1")
            .bind(token)
            .execute(self.pool())
            .await?;
        Ok(())
    }
}

pub fn create_token_service(pool: PostgresPool) -> Box<dyn TokenService> {
    Box::new(pool)
}
