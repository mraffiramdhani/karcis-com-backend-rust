mod app;
mod db;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server_data = db::connection_builder().await.unwrap();
    let _migrate_result = sqlx::migrate!("./migrations")
        .run(&server_data)
        .await
        .unwrap();

    let server_host = dotenv::var("SERVER_HOST").unwrap();
    let server_port = dotenv::var("SERVER_PORT").unwrap();
    let server_location = server_host + ":" + &server_port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(server_data.clone()))
            .configure(app::register_urls)
    })
    .bind(&server_location)?
    .run()
    .await
}
