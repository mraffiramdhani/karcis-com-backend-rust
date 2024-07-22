use super::views::{get_balance, get_balance_histories, update_balance};
use actix_web::web;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/balance")
            .route("/{balance_id}", web::get().to(get_balance))
            .route("/history/{user_id}", web::get().to(get_balance_histories))
            .route("/{balance_id}", web::put().to(update_balance)),
    );
}
