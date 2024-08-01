use crate::app::auth::models::{LoginCredentials, Profile, RegisterUser, Token, User};
use crate::app::balance::models::Balance;
use crate::app::mail_template::forgot_password::template;
use crate::app::otp::models::OTP;
use crate::app::utils::generator::generateOTP;
use crate::app::utils::mail::send_mail;
use crate::app::utils::{standard_response::StandardResponse, token_signing::TokenSigning};
use crate::db::DbPool;
use actix_web::HttpRequest;
use actix_web::{http::header, web, HttpResponse, Responder};
use serde_json::json;

use super::models::ForgotPassword;

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    // Check if user with the same email or username already exists
    match User::find_by_username_or_email(&pool, &user_data.username, &user_data.email).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(StandardResponse::<()>::error(
                "User with this username or email already exists".into(),
            ))
        }
        Ok(None) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                format!("Find Existing User Error: {}", e.to_string()),
            ))
        }
    }

    match pool.begin().await {
        Ok(mut tx) => match User::create(&mut tx, &user_data).await {
            Ok(user) => match Balance::create(&mut tx, &user.id).await {
                Ok(_) => {
                    let token_data = TokenSigning {
                        id: user.id.clone(),
                        first_name: user.first_name.clone(),
                        last_name: user.last_name.clone(),
                        email: user.email.clone(),
                    };
                    let token = TokenSigning::sign(token_data).unwrap();
                    match Token::create(&mut tx, &token).await {
                        Ok(_) => match tx.commit().await {
                            Ok(()) => {
                                let profile = Profile {
                                    first_name: user.first_name.clone(),
                                    last_name: user.last_name.clone(),
                                    phone: user.phone.clone(),
                                    id: user.id.clone(),
                                    username: user.username.clone(),
                                    email: user.email.clone(),
                                };
                                HttpResponse::Created().json(StandardResponse::ok(
                                    json!({"profile": profile, "token": &token}),
                                    Some("Profile created successfully.".into()),
                                ))
                            }
                            Err(e) => HttpResponse::InternalServerError().json(
                                StandardResponse::<()>::error(format!(
                                    "DB Transaction Commit Error: {}",
                                    e.to_string()
                                )),
                            ),
                        },
                        Err(e) => {
                            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                                format!("Token Create Error: {}", e.to_string()),
                            ))
                        }
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    format!("Balance Create Error: {}", e.to_string()),
                )),
            },
            Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                format!("User Create Error: {}", e.to_string()),
            )),
        },
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "DB Transaction Initialization Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginCredentials>,
) -> impl Responder {
    match User::find_by_username(&pool, &credentials.username).await {
        Ok(user) => {
            if user.is_none() {
                return HttpResponse::Unauthorized()
                    .json(StandardResponse::<()>::error("Invalid credentials".into()));
            }
            let user = user.unwrap();
            if !user.verify_password(&credentials.password) {
                return HttpResponse::Unauthorized()
                    .json(StandardResponse::<()>::error("Invalid credentials".into()));
            }
            let data = TokenSigning {
                id: user.id.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
            };
            let token_string = TokenSigning::sign(data).unwrap();
            match pool.begin().await {
                Ok(mut tx) => match Token::create(&mut tx, &token_string).await {
                    Ok(_) => match tx.commit().await {
                        Ok(()) => {
                            let profile = Profile {
                                first_name: user.first_name.clone(),
                                last_name: user.last_name.clone(),
                                phone: user.phone.clone(),
                                id: user.id.clone(),
                                username: user.username.clone(),
                                email: user.email.clone(),
                            };
                            HttpResponse::Ok().json(StandardResponse::ok(
                                json!({"profile": profile,"token": token_string}),
                                Some("Login success.".to_string()),
                            ))
                        }
                        Err(e) => {
                            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                                format!("DB Transaction Commit Error: {}", e.to_string()),
                            ))
                        }
                    },
                    Err(e) => {
                        HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                            format!("Token Create Error: {}", e.to_string()),
                        ))
                    }
                },
                Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    format!("DB Transaction Initialization Error: {}", e.to_string()),
                )),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(e.to_string()))
        }
    }
}

pub async fn logout(pool: web::Data<DbPool>, header: HttpRequest) -> impl Responder {
    // TODO: Implement logout logic
    let token = header
        .headers()
        .get(header::AUTHORIZATION)
        .unwrap()
        .as_bytes();
    let token_str = std::str::from_utf8(token)
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .to_string();
    match pool.begin().await {
        Ok(mut tx) => match Token::revoke_token(&mut tx, token_str).await {
            Ok(rows_affected) => HttpResponse::Ok().json(StandardResponse::ok(
                rows_affected,
                Some(format!("rows affected: {}", rows_affected)),
            )),
            Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                format!("Token Revoke Error: {}", e.to_string()),
            )),
        },
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "DB Transaction Initialization Error: {}",
            e.to_string()
        ))),
    }
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

pub async fn forgot_password(
    pool: web::Data<DbPool>,
    data: web::Json<ForgotPassword>,
) -> impl Responder {
    match User::find_by(&pool, "email", &data.email.as_str()).await {
        Ok(Some(user)) => {
            let otp_codes = generateOTP().to_string();
            match pool.begin().await {
                Ok(mut tx) => match OTP::create(&mut tx, &otp_codes.as_str(), 1).await {
                    Ok(_) => match send_mail(user, template(&otp_codes.as_str())).await {
                        Ok(_) => HttpResponse::Ok().json(StandardResponse::ok(
                            (),
                            Some("Email sent successfully.".to_string()),
                        )),
                        Err(e) => HttpResponse::Conflict().json(StandardResponse::<()>::error(
                            format!("Send Mail Error: {}", e.to_string()),
                        )),
                    },
                    Err(e) => HttpResponse::Conflict().json(StandardResponse::<()>::error(
                        format!("OTP Create Error: {}", e.to_string()),
                    )),
                },
                Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    format!("DB Transaction Initialization Error: {}", e.to_string()),
                )),
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("User not found".into()))
        }
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Forgot Password Error: {}",
            e.to_string()
        ))),
    }
}
