use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::*;
use sqlx::{FromRow, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Balance {
    pub id: i64,
    pub user_id: i64,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBalance {
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBalance {
    #[serde(with = "bigdecimal::serde::json_num")]
    pub balance: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BalanceHistory {
    pub id: i64,
    pub user_id: i64,
    pub balance_id: i64,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub balance: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub top_up: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBalanceHistory {
    pub user_id: i64,
    pub balance_id: i64,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub balance: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub top_up: BigDecimal,
}

impl Balance {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &i64,
    ) -> Result<Self, sqlx::Error> {
        let balance = sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO balances (user_id, balance)
            VALUES ($1, 0)
            RETURNING *
            "#,
        )
        .bind(&user_id)
        .fetch_one(&mut **transaction)
        .await?;

        Ok(balance)
    }

    pub async fn get(db: &sqlx::PgPool, id: &i64) -> Result<Self, sqlx::Error> {
        let balance = sqlx::query_as::<_, Balance>(
            r#"
            SELECT id, user_id, balance, created_at, updated_at
            FROM balances
            WHERE id = $1
            "#,
        )
        .bind(&id)
        .fetch_one(db)
        .await?;

        Ok(balance)
    }

    pub async fn get_by_user(db: &sqlx::PgPool, user_id: i64) -> Result<Self, sqlx::Error> {
        let balance = sqlx::query_as::<_, Self>(
            r#"
            SELECT id, user_id, balance, created_at, updated_at
            FROM balances
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(db)
        .await?;

        Ok(balance)
    }

    pub async fn update(
        db: &sqlx::PgPool,
        id: &i64,
        update: &UpdateBalance,
    ) -> Result<Self, sqlx::Error> {
        let balance = sqlx::query_as::<_, Self>(
            r#"
            UPDATE balances
            SET balance = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING id, user_id, balance, created_at, updated_at
            "#,
        )
        .bind(&update.balance)
        .bind(&id)
        .fetch_one(db)
        .await?;

        Ok(balance)
    }

    pub async fn delete(db: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM balances
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }
}

impl BalanceHistory {
    pub async fn create(
        db: &sqlx::PgPool,
        new_balance_history: CreateBalanceHistory,
    ) -> Result<Self, sqlx::Error> {
        let balance_history = sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO balance_histories (user_id, balance_id, balance, top_up)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, balance_id, balance, top_up, created_at
            "#,
        )
        .bind(new_balance_history.user_id)
        .bind(new_balance_history.balance_id)
        .bind(new_balance_history.balance)
        .bind(new_balance_history.top_up)
        .fetch_one(db)
        .await?;

        Ok(balance_history)
    }

    pub async fn get(db: &sqlx::PgPool, id: i64) -> Result<Self, sqlx::Error> {
        let balance_history = sqlx::query_as::<_, Self>(
            r#"
            SELECT id, user_id, balance_id, balance, top_up, created_at
            FROM balance_histories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(balance_history)
    }

    pub async fn get_by_user(db: &sqlx::PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        let balance_histories = sqlx::query_as::<_, Self>(
            r#"
            SELECT id, user_id, balance_id, balance, top_up, created_at
            FROM balance_histories
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        Ok(balance_histories)
    }
}
