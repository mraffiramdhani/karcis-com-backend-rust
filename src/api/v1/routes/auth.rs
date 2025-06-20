use actix_web::web;

use crate::api::v1::handlers::auth;
use crate::domain::middlewares::auth::Authorization;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth::login))
            .route("/register", web::post().to(auth::register))
            .route(
                "/forgot-password",
                web::post().to(auth::forgot_password),
            )
            .route(
                "/test-email",
                web::get().to(auth::test_email_connection),
            )
            // .route(
            //     "/forgot-password/reset",
            //     web::post().to(super::views::reset_password),
            // )
            // .route("/otp-check", web::post().to(super::views::check_otp))
            .route(
                "/logout",
                web::get().to(auth::logout).wrap(Authorization::require_user()),
            ),
    );
    // .service(
    //     web::scope("/u")
    //         .wrap(Authentication)
    //         .route(
    //             "/profile/{user_id}",
    //             web::get().to(super::views::get_profile),
    //         )
    //         .route("/profile", web::patch().to(super::views::update_profile)),
    // );
}
