pub type DbPool = sqlx::postgres::PgPool;

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = dotenv::var("DATABASE_URL").unwrap();
    sqlx::postgres::PgPool::connect(&connectspec).await
}
