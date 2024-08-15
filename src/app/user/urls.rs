use actix_web::{dev::Service, error::Error as ActixError, web};

use crate::app::middlewares::has_role::HasRole;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(HasRole::new("admin".to_string()))
            .route("/{id}", web::get().to(super::views::get_user_by_id)),
    );
}
