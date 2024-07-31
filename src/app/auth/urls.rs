use actix_web::{guard, web};

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(super::views::login))
            .route("/register", web::post().to(super::views::register))
            .route("/logout", web::post().to(super::views::logout))
            .route("/profile", web::get().to(super::views::get_profile)),
    );
}
