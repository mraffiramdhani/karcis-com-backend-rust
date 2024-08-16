use actix_web::web;

use crate::app::middlewares::auth::Authentication;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(super::views::login))
            .route("/register", web::post().to(super::views::register))
            .route(
                "/forgot-password",
                web::post().to(super::views::forgot_password),
            )
            .route(
                "/forgot-password/reset",
                web::post().to(super::views::reset_password),
            )
            .route("/otp-check", web::post().to(super::views::check_otp))
            .route(
                "/logout",
                web::get().to(super::views::logout).wrap(Authentication),
            ),
    )
    .service(
        web::scope("/u")
            .wrap(Authentication)
            .route(
                "/profile/{user_id}",
                web::get().to(super::views::get_profile),
            )
            .route("/profile", web::patch().to(super::views::update_profile)),
    );
}
