use sqlx::postgres::PgPoolOptions;
pub type DbPool = sqlx::postgres::PgPool;

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = dotenv::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&connectspec)
        .await
}
