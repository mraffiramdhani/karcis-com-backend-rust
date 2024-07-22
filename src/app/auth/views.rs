use crate::app::auth::models::{LoginCredentials, Profile, RegisterUser, TokenPair, User};
use crate::app::balance::models::{Balance, CreateBalance};
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};

pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginCredentials>,
) -> impl Responder {
    let user = User::find_by_username(&pool, &credentials.username)
        .await
        .expect("Failed to query user by username");
    if user.is_none() {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }
    let user = user.unwrap();
    if !user.verify_password(&credentials.password) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }
    let token_pair = TokenPair::new(user.id);
    HttpResponse::Ok().json(token_pair)
}

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    match User::create(&pool, &user_data).await {
        Ok(user) => {
            let new_balance = CreateBalance { user_id: user.id };
            match Balance::create(&pool, new_balance).await {
                Ok(_) => {
                    let profile = Profile {
                        first_name: user.first_name,
                        last_name: user.last_name,
                        phone: user.phone,
                        id: user.id,
                        username: user.username,
                        email: user.email,
                    };
                    HttpResponse::Created().json(profile)
                }
                Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Could not create user"),
    }
}

pub async fn logout() -> impl Responder {
    // TODO: Implement logout logic
    // 1. Invalidate tokens
    HttpResponse::Ok().json("Logout functionality to be implemented")
}

pub async fn get_profile(
    pool: web::Data<DbPool>,
    // TODO: Add authentication middleware to extract user_id
    user_id: web::Path<i64>,
) -> impl Responder {
    match User::find_by_id(&pool, user_id.into_inner()).await {
        Ok(Some(user)) => {
            let profile = Profile {
                first_name: user.first_name,
                last_name: user.last_name,
                phone: user.phone,
                id: user.id,
                username: user.username,
                email: user.email,
            };
            HttpResponse::Ok().json(profile)
        }
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Could not retrieve user"),
    }
}

pub async fn refresh_token(_pool: web::Data<DbPool>, user_id: web::Path<i64>) -> impl Responder {
    let token_pair = TokenPair::new(user_id.into_inner());
    HttpResponse::Ok().json(token_pair)
}
