use actix_web::{dev::ServiceRequest, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{
    domain::{
        middlewares::auth::get_current_token,
        models::{
            auth::{ForgotPasswordPayload, LoginPayload, RegisterPayload},
            StandardResponse, User,
        },
        services::{auth::create_auth_service, token::create_token_service, user::create_user_service},
        validations::auth_validations::{AuthValidator, ValidationError},
    },
    infrastructure::{database::PostgresPool, email::test_smtp_connection},
    shared::utils::error_helpers::{
        handle_database_error, handle_error, handle_invalid_credentials, handle_validation_error,
    },
};

pub async fn register(
    pool: web::Data<PostgresPool>,
    user_data: web::Json<RegisterPayload>,
) -> impl Responder {
    //* Check if form submitted is valid
    if let Err(e) = AuthValidator::validate_register_payload(&user_data) {
        return match e {
            ValidationError::Single(error) => handle_validation_error(vec![error]),
            ValidationError::Multiple(errors) => handle_validation_error(errors),
        };
    }
    //*  Check if user with the same email or username already exists
    let user_service = create_user_service(pool.get_ref().clone());
    match user_service.find_by("username", &user_data.username).await {
        Ok(Some(_)) => {
            return handle_validation_error(vec![
                "User with this username or email already exists".into()
            ])
        }
        Ok(None) => {}
        Err(e) => {
            return handle_database_error::<User>(e, "Find Existing User");
        }
    }

    //* Check if user with the same email already exists
    match user_service.find_by("email", &user_data.email).await {
        Ok(Some(_)) => {
            return handle_validation_error(vec!["User with this email already exists".into()])
        }
        Ok(None) => {}
        Err(e) => {
            return handle_database_error::<User>(e, "Find Existing User");
        }
    }

    match user_service.create(&user_data).await {
        Ok((user, token)) => {
            return HttpResponse::Created().json(StandardResponse::ok(
                json!({"profile": user, "token": &token}),
                Some("User created successfully.".into()),
            ))
        }
        Err(e) => {
            return handle_database_error::<User>(e, "Create User");
        }
    }
}

pub async fn login(
    pool: web::Data<PostgresPool>,
    login_data: web::Json<LoginPayload>,
) -> impl Responder {
    //* Check if form submitted is valid
    if let Err(e) = AuthValidator::validate_login_payload(&login_data) {
        return match e {
            ValidationError::Single(error) => handle_validation_error(vec![error]),
            ValidationError::Multiple(errors) => handle_validation_error(errors),
        };
    }
    //* Check if user exists
    let user_service = create_user_service(pool.get_ref().clone());
    match user_service.login(&login_data).await {
        Ok((user, token)) => {
            return HttpResponse::Ok().json(StandardResponse::ok(
                json!({"profile": user, "token": &token}),
                Some("User logged in successfully.".into()),
            ))
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                return handle_invalid_credentials();
            }
            _ => {
                return handle_database_error::<User>(e, "Login User");
            }
        },
    }
}

pub async fn logout(pool: web::Data<PostgresPool>, req: HttpRequest) -> impl Responder {
    // Extract token from request extensions (set by Authorization middleware)
    let token = match get_current_token(&ServiceRequest::from_request(req)) {
        Some(token) => token,
        None => {
            return HttpResponse::Unauthorized().json(StandardResponse::<()>::error(
                "No valid token found".to_string(),
                Some("UNAUTHORIZED".to_string()),
            ));
        }
    };

    let token_service = create_token_service(pool.get_ref().clone());
    match token_service.revoke_token(&token).await {
        Ok(_) => {
            return HttpResponse::Ok().json(StandardResponse::ok(
                json!({"message": "Logged out successfully."}),
                Some("Token revoked successfully.".into()),
            ));
        }
        Err(e) => {
            return handle_database_error::<()>(e, "Revoke Token");
        }
    }
}

pub async fn forgot_password(
    pool: web::Data<PostgresPool>,
    data: web::Json<ForgotPasswordPayload>,
) -> impl Responder {
    let auth_service = create_auth_service(pool.get_ref().clone());
    match auth_service.forgot_password(&data).await {
        Ok(_) => {
            return HttpResponse::Ok().json(StandardResponse::ok(json!({"message": "Email sent successfully."}), Some("Email sent successfully.".into())));
        }
        Err(e) => { 
            return handle_error::<()>(e, "Forgot Password");
        }
    }
}

pub async fn test_email_connection() -> impl Responder {
    match test_smtp_connection().await {
        Ok(_) => {
            HttpResponse::Ok().json(StandardResponse::ok(
                json!({"message": "SMTP connection test successful"}),
                Some("SMTP connection test successful".into()),
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                format!("SMTP connection test failed: {}", e),
                Some("SMTP_TEST_FAILED".to_string()),
            ))
        }
    }
}
