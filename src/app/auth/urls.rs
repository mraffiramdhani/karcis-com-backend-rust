use actix_web::web;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(super::views::login))
            .route("/register", web::post().to(super::views::register))
            .route("/logout", web::post().to(super::views::logout))
            .route(
                "/forgot-password",
                web::post().to(super::views::forgot_password),
            ),
    );
}
