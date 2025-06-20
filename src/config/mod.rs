use serde::Deserialize;
use std::env;

mod database;
mod server;
mod email;
// mod jwt;

pub use database::DatabaseSettings;
pub use server::ServerSettings;
pub use email::EmailSettings;
// pub use jwt::JwtSettings;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub email: EmailSettings,
    // pub jwt: JwtSettings,
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let config = config::Config::builder()
            // Start with default settings
            .add_source(config::File::with_name("config/default"))
            // Add environment-specific settings
            .add_source(config::File::with_name(&format!("config/{}", environment)).required(false))
            // Add environment variables (with prefix APP_)
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}