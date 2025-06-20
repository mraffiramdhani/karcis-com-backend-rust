use sqlx::migrate::{MigrateError};
use sqlx::PgPool;
use tracing::{info};

/// Represents the current state of database migrations
#[derive(Debug)]
pub struct MigrationState {
    pub current_version: i64,
    pub latest_version: i64,
}

/// Runs all pending migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    let migrations = sqlx::migrate!("./migrations");

    // Get current migration state
    let state = get_migration_state(pool).await?;

    if state.current_version == state.latest_version {
        info!(
            "Database is up to date at version {}",
            state.current_version
        );
        return Ok(());
    }

    info!(
        "Running {} pending migrations ({} -> {})",
        state.latest_version - state.current_version,
        state.current_version,
        state.latest_version
    );

    // Run migrations
    migrations.run(pool).await?;

    info!("Successfully ran all pending migrations");
    Ok(())
}

/// Gets the current state of database migrations
pub async fn get_migration_state(pool: &PgPool) -> Result<MigrationState, MigrateError> {
    let migrations = sqlx::migrate!("./migrations");
    
    // Check if _sqlx_migrations table exists first
    let current_version = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(MAX(version), 0) FROM _sqlx_migrations"
    )
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
    .unwrap_or(0);

    let latest_version = migrations.iter().map(|m| m.version).max().unwrap_or(0);

    Ok(MigrationState {
        current_version,
        latest_version,
    })
}

//* Reverts the last migration
// pub async fn revert_last_migration(pool: &PgPool) -> Result<(), MigrateError> {
//     let state = get_migration_state(pool).await?;

//     if state.current_version == 0 {
//         warn!("No migrations to revert");
//         return Ok(());
//     }

//     info!(
//         "Reverting migration to version {}",
//         state.current_version - 1
//     );

//     let migrations = sqlx::migrate!("./migrations");
//     migrations.undo(pool, state.current_version - 1).await?;

//     info!("Successfully reverted last migration");
//     Ok(())
// }

//* Checks if the database is up to date
// pub async fn is_database_up_to_date(pool: &PgPool) -> Result<bool, MigrateError> {
//     let state = get_migration_state(pool).await?;
//     Ok(state.current_version == state.latest_version)
// }

//* Validates that all migrations can be applied
// pub async fn validate_migrations(pool: &PgPool) -> Result<(), MigrateError> {
//     let migrations = sqlx::migrate!("./migrations");
//     migrations.run(pool).await?;
//     Ok(())
// }
