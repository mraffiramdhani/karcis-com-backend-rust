pub mod auth;
pub mod balance;
pub mod utils;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    auth::urls::register_urls(cfg);
    balance::urls::register_urls(cfg);
}
