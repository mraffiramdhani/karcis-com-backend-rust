mod migrations;
mod postgres;
// mod repositories;

pub use migrations::run_migrations;
pub use postgres::PostgresPool;

use crate::config::DatabaseSettings;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Initialize a new PostgreSQL connection pool
pub async fn init_pool(settings: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .connect(&settings.connection_string())
        .await
}
