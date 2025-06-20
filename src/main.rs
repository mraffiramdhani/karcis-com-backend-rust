mod api;
mod config;
mod domain;
mod infrastructure;
mod shared;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use infrastructure::database::{init_pool, run_migrations, PostgresPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let settings = config::Settings::load().expect("Failed to load configuration");

    // Initialize database
    let pool = init_pool(&settings.database)
        .await
        .expect("Failed to initialize database pool");

    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // Create database infrastructure
    let db_pool = PostgresPool::new(pool);

    // Start server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_header().allow_any_method())
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .configure(api::register_urls)
    })
    .bind(settings.server.address())?
    .run()
    .await
}
