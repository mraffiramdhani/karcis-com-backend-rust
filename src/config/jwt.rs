use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    #[serde(default = "default_expiration")]
    pub expiration_hours: u64,
    #[serde(default = "default_issuer")]
    pub issuer: String,
}

fn default_expiration() -> u64 {
    24
}

fn default_issuer() -> String {
    "karcis-com".to_string()
}

impl JwtSettings {
    pub fn expiration(&self) -> Duration {
        Duration::from_secs(self.expiration_hours * 3600)
    }
} 