use dotenv::var;
use sqlx::postgres::PgPoolOptions;
pub type DbPool = sqlx::postgres::PgPool;

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connectspec)
        .await
}
