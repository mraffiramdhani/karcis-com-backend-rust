use actix_web::{web, HttpResponse, Responder};

use crate::{
    app::{
        auth::models::{Profile, User},
        utils::standard_response::StandardResponse,
    },
    db::DbPool,
};

pub async fn get_user_by_id(pool: web::Data<DbPool>, id: web::Path<i64>) -> impl Responder {
    match User::find_by_id(&pool, id.into_inner()).await {
        Ok(Some(user)) => {
            let profile = Profile {
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                phone: user.phone.clone(),
                id: user.id.clone(),
                username: user.username.clone(),
                email: user.email.clone(),
                image: user.image.clone(),
                title: user.title.clone(),
                role_id: user.role_id.clone(),
            };
            HttpResponse::Ok().json(StandardResponse::ok(profile, Some("User found.".into())))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("User not found.".into()))
        }
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Get User Error: {}",
            e.to_string()
        ))),
    }
}
