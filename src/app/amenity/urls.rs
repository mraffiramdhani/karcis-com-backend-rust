use actix_web::web;

use crate::app::middlewares::{auth::Authentication, has_role::HasRole};

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/amenity")
            .route("/", web::get().to(super::views::get_all_amenities))
            .route("/{id}", web::get().to(super::views::get_amenity_by_id))
            .route(
                "/",
                web::post()
                    .to(super::views::create_amenity)
                    .wrap(Authentication)
                    .wrap(HasRole::new("admin".to_string())),
            )
            .route(
                "/",
                web::patch()
                    .to(super::views::update_amenity)
                    .wrap(Authentication)
                    .wrap(HasRole::new("admin".to_string())),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(super::views::delete_amenity)
                    .wrap(Authentication)
                    .wrap(HasRole::new("admin".to_string())),
            ),
    );
}
