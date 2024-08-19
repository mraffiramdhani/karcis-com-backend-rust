pub mod amenity;
pub mod auth;
pub mod balance;
pub mod mail_template;
pub mod middlewares;
pub mod otp;
pub mod user;
pub mod utils;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    amenity::urls::register_urls(cfg);
    auth::urls::register_urls(cfg);
    user::urls::register_urls(cfg);
    // balance::urls::register_urls(cfg);
}
