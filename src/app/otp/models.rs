use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OTP {
    pub id: i64,
    pub code: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
}

impl OTP {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        code: &str,
        expiry_in_min: u8,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(format!("INSERT INTO otp_codes (code, expired_at) VALUES ($1, CURRENT_DATE + INTERVAL '{} minute')", &expiry_in_min).as_str()).bind(&code).execute(&mut **transaction).await;
        match result {
            Ok(res) => Ok(res.rows_affected()),
            Err(e) => Err(e),
        }
    }

    pub async fn check_code(pool: &PgPool, code: &str) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM otp_codes WHERE code = $1")
            .bind(code)
            .fetch_optional(pool)
            .await;
        result
    }

    pub async fn revoke_code(pool: &PgPool, code: &str) -> Result<u64, sqlx::Error> {
        match sqlx::query("UPDATE otp_codes SET is_active = 0 WHERE code = $1 AND is_active = 1 AND expired_at >= CURRENT_DATE")
            .bind(code)
            .execute(pool)
            .await
        {
            Ok(row) => Ok(row.rows_affected()),
            Err(e) => Err(e),
        }
    }
}
