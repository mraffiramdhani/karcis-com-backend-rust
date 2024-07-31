use crate::app::auth::models::{LoginCredentials, Profile, RegisterUser, Token, User};
use crate::app::balance::models::{Balance, CreateBalance};
use crate::app::utils::{StandardResponse, TokenSigning};
use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginCredentials>,
) -> impl Responder {
    let user = User::find_by_username_or_email(&pool, &credentials.username, "")
        .await
        .expect("Failed to query user by username");
    if user.is_none() {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }
    let user = user.unwrap();
    if !user.verify_password(&credentials.password) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }
    let data = TokenSigning {
        id: user.id,
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
    };
    let token_string = TokenSigning::sign(data).unwrap();
    match Token::create(&pool, &token_string).await {
        Ok(_) => HttpResponse::Ok().json(json!({"token": token_string})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    // Check if user with the same email or username already exists
    let existing_user =
        User::find_by_username_or_email(&pool, &user_data.username, &user_data.email).await;
    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json("User with this username or email already exists")
        }
        Ok(None) => {}
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }

    match User::create(&pool, &user_data).await {
        Ok(user) => {
            let new_balance = CreateBalance { user_id: user.id };
            match Balance::create(&pool, new_balance).await {
                Ok(_) => {
                    // let token_data = TokenSigning {
                    //     id: user.id,
                    //     first_name: user.first_name.clone(),
                    //     last_name: user.last_name.clone(),
                    //     email: user.email.clone(),
                    // };
                    // let token = TokenSigning::sign(token_data).unwrap();
                    // match Token::create(&pool, &token).await {
                    //     Ok(_) => {
                    let profile = Profile {
                        first_name: user.first_name,
                        last_name: user.last_name,
                        phone: user.phone,
                        id: user.id,
                        username: user.username,
                        email: user.email,
                    };
                    HttpResponse::Created().json(StandardResponse::ok(
                        profile,
                        Some("Profile created successfully.".into()),
                    ))
                    //     }
                    //     Err(e) => HttpResponse::InternalServerError()
                    //         .json(StandardResponse::<()>::error(e.to_string())),
                    // }
                }
                Err(e) => HttpResponse::InternalServerError()
                    .json(StandardResponse::<()>::error(e.to_string())),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn logout() -> impl Responder {
    // TODO: Implement logout logic
    HttpResponse::Ok().json("Logout functionality to be implemented")
}

pub async fn get_profile(pool: web::Data<DbPool>, user_id: web::Path<i64>) -> impl Responder {
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
