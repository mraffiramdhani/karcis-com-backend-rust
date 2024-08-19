use crate::app::auth::models::{LoginCredentials, Profile, RegisterUser, Token, User};
use crate::app::balance::models::Balance;
use crate::app::mail_template::forgot_password::template;
use crate::app::otp::models::OTP;
use crate::app::utils::generator::generate_otp;
use crate::app::utils::mail::send_mail;
use crate::app::utils::{standard_response::StandardResponse, token_signing::TokenSigning};
use crate::db::DbPool;
use actix_web::HttpRequest;
use actix_web::{http::header, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde_json::json;

use super::models::{CheckOTP, ForgotPassword, ResetPassword, UpdateProfile};

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
                        role_id: user.role_id.clone(),
                        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
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
                                    title: user.title.clone(),
                                    image: user.image.clone(),
                                    role_id: user.role_id.clone(),
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
                role_id: user.role_id.clone(),
                exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
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
                                image: user.image.clone(),
                                title: user.title.clone(),
                                role_id: user.role_id.clone(),
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
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Find By Username Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn logout(pool: web::Data<DbPool>, header: HttpRequest) -> impl Responder {
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
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    match tx.commit().await {
                        Ok(()) => HttpResponse::Ok().json(StandardResponse::ok(
                            rows_affected,
                            Some(format!("rows affected: {}", rows_affected)),
                        )),
                        Err(e) => {
                            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                                format!("DB Transaction Commit Error: {}", e.to_string()),
                            ))
                        }
                    }
                } else {
                    HttpResponse::Unauthorized().finish()
                }
            }
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
            HttpResponse::Ok().json(StandardResponse::ok(
                profile,
                Some("User found".to_string()),
            ))
        }
        Ok(None) => HttpResponse::NotFound()
            .json(StandardResponse::<()>::error("User not found.".to_string())),
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Find User By Id Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn update_profile(
    pool: web::Data<DbPool>,
    data: web::Json<UpdateProfile>,
) -> impl Responder {
    match pool.begin().await {
        Ok(mut transaction) => match User::update(&mut transaction, &data).await {
            Ok(user) => match transaction.commit().await {
                Ok(()) => {
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
                    HttpResponse::Ok().json(StandardResponse::ok(
                        profile,
                        Some("Update profile success.".to_string()),
                    ))
                }
                Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                    format!("DB Transaction Commit Error: {}", e.to_string()),
                )),
            },
            Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                format!("Update User Error: {}", e.to_string()),
            )),
        },
        Err(e) => HttpResponse::InternalServerError().json(format!(
            "DB Transaction Initialization Error: {}",
            e.to_string()
        )),
    }
}

pub async fn forgot_password(
    pool: web::Data<DbPool>,
    data: web::Json<ForgotPassword>,
) -> impl Responder {
    match User::find_by(&pool, "email", &data.email.as_str()).await {
        Ok(Some(user)) => {
            let otp_codes = generate_otp().to_string();
            match pool.begin().await {
                Ok(mut tx) => match OTP::create(&mut tx, &otp_codes.as_str(), 5).await {
                    Ok(_) => match send_mail(
                        user,
                        "You've requested to reset your password",
                        template(&otp_codes.as_str()),
                    )
                    .await
                    {
                        Ok(_) => match tx.commit().await {
                            Ok(()) => HttpResponse::Ok().json(StandardResponse::ok(
                                (),
                                Some("Email sent successfully.".to_string()),
                            )),
                            Err(e) => HttpResponse::InternalServerError().json(
                                StandardResponse::<()>::error(format!(
                                    "DB Transaction Commit Error: {}",
                                    e.to_string()
                                )),
                            ),
                        },
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

pub async fn check_otp(pool: web::Data<DbPool>, data: web::Json<CheckOTP>) -> impl Responder {
    let code = &data.code;
    match OTP::check_code(&pool, &code.as_str()).await {
        Ok(Some(_res)) => match OTP::revoke_code(&pool, &code.as_str()).await {
            Ok(res) => {
                if res > 0 {
                    HttpResponse::Ok().json(StandardResponse::ok(
                        (),
                        Some("OTP check success. Your code is valid".to_string()),
                    ))
                } else {
                    HttpResponse::Conflict().json(StandardResponse::<()>::error(
                        "OTP check failed. Your code is expired or invalid, please try again."
                            .to_string(),
                    ))
                }
            }
            Err(e) => HttpResponse::Conflict().json(StandardResponse::<()>::error(format!(
                "Revoke Code Error: {}",
                e.to_string()
            ))),
        },
        Ok(None) => HttpResponse::NotFound()
            .json(StandardResponse::<()>::error("OTP code not found.".into())),
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Check OTP Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn reset_password(
    pool: web::Data<DbPool>,
    data: web::Json<ResetPassword>,
) -> impl Responder {
    match User::find_by(&pool, "email", &data.email.as_str()).await {
        Ok(Some(_user)) => {
            let new_password = &data.new_password;
            let confirm_password = &data.confirm_password;
            if &new_password == &confirm_password {
                match pool.begin().await {
                    Ok(mut transaction) => {
                        match User::update_password(&mut transaction, &data.into_inner()).await {
                            Ok(_res) => match transaction.commit().await {
                                Ok(()) => HttpResponse::Ok().json(StandardResponse::ok(
                                    (),
                                    Some("Reset password success.".into()),
                                )),
                                Err(e) => HttpResponse::InternalServerError().json(
                                    StandardResponse::<()>::error(format!(
                                        "DB Transaction Commit Error: {}",
                                        e.to_string()
                                    )),
                                ),
                            },
                            Err(e) => HttpResponse::InternalServerError().json(
                                StandardResponse::<()>::error(format!(
                                    "Update User Error: {}",
                                    e.to_string()
                                )),
                            ),
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().json(format!(
                        "DB Transaction Initialization Error: {}",
                        e.to_string()
                    )),
                }
            } else {
                HttpResponse::Conflict().json(StandardResponse::<()>::error(
                    "Password not match. Please try again.".into(),
                ))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(StandardResponse::<()>::error(
            "User not found. Please try again.".into(),
        )),
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Check Existing User Error: {}",
            e.to_string()
        ))),
    }
}
