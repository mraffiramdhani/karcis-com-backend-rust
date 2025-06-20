mod v1;

use actix_web::web;

pub fn register_urls(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/v1").configure(v1::routes::auth::register_urls)),
    );
}
