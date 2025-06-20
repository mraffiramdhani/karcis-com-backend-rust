use async_trait::async_trait;

use crate::{domain::services::OtpService, infrastructure::database::PostgresPool};

#[async_trait]
impl OtpService for PostgresPool {
    async fn create(&self, otp: &str) -> Result<(), sqlx::Error> {
        sqlx::query(format!("INSERT INTO otp_codes (code, expired_at) VALUES ($1, CURRENT_TIMESTAMP + INTERVAL '{} minute')", 5).as_str())
        .bind(&otp)
        .execute(self.pool())
        .await?;
        Ok(())
    }
}

pub fn create_otp_service(pool: PostgresPool) -> Box<dyn OtpService> {
    Box::new(pool)
}
