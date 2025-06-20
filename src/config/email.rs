use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    #[serde(default = "default_use_tls")]
    pub use_tls: bool,
}

fn default_use_tls() -> bool {
    true
}