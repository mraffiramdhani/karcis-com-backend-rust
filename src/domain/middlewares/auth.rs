use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    web::{self},
    Error, HttpMessage, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models::user::User, domain::services::TokenService,
    infrastructure::database::PostgresPool,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i32,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthorizationConfig {
    pub required_roles: Vec<i32>,
    pub check_permissions: bool,
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            required_roles: vec![],
            check_permissions: false,
        }
    }
}

pub struct Authorization {
    config: AuthorizationConfig,
}

impl Authorization {
    pub fn new(config: AuthorizationConfig) -> Self {
        Self { config }
    }

    pub fn require_roles(roles: Vec<i32>) -> Self {
        Self {
            config: AuthorizationConfig {
                required_roles: roles,
                check_permissions: false,
            },
        }
    }

    pub fn require_admin() -> Self {
        Self::require_roles(vec![1]) // Assuming role_id 1 is admin
    }

    pub fn require_user() -> Self {
        Self::require_roles(vec![2]) // Assuming role_id 2 is regular user
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware {
            service: Rc::new(service),
            config: self.config.clone(),
        }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: Rc<S>,
    config: AuthorizationConfig,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(AUTHORIZATION);
        if auth_header.is_none() {
            let http_res = HttpResponse::Unauthorized().json(serde_json::json!({
                "status": "error",
                "message": "Authorization header is required"
            }));
            let (http_req, _) = req.into_parts();
            let res = ServiceResponse::new(http_req, http_res);
            return (async move { Ok(res.map_into_right_body()) }).boxed_local();
        }

        let auth_value = auth_header.unwrap().to_str().unwrap();
        if !auth_value.starts_with("Bearer ") {
            let http_res = HttpResponse::Unauthorized().json(serde_json::json!({
                "status": "error",
                "message": "Invalid authorization header format. Expected 'Bearer <token>'"
            }));
            let (http_req, _) = req.into_parts();
            let res = ServiceResponse::new(http_req, http_res);
            return (async move { Ok(res.map_into_right_body()) }).boxed_local();
        }

        let token = auth_value[7..].to_string(); // Remove "Bearer " prefix
        let config = self.config.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Get database pool
            let pool = req.app_data::<web::Data<PostgresPool>>().unwrap().get_ref();

            // Check if token is revoked
            if TokenService::is_token_revoked(pool, &token)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
            {
                let http_res = HttpResponse::Unauthorized().json(serde_json::json!({
                    "status": "error",
                    "message": "Token has been revoked"
                }));
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body());
            }

            // Verify and decode JWT token
            let secret = std::env::var("APP_KEY")
                .map_err(|_| actix_web::error::ErrorInternalServerError("APP_KEY not found"))?;

            let token_data = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))?;

            let claims = token_data.claims;

            // Check if user exists and is not deleted
            let user = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(claims.id)
            .fetch_optional(pool.pool())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            if user.is_none() {
                let http_res = HttpResponse::Unauthorized().json(serde_json::json!({
                    "status": "error",
                    "message": "User not found or has been deleted"
                }));
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body());
            }

            let user = user.unwrap();

            // Check role-based authorization
            if !config.required_roles.is_empty() && !config.required_roles.contains(&user.role_id) {
                let http_res = HttpResponse::Forbidden()
                    .json(serde_json::json!({
                        "status": "error",
                        "message": format!("Insufficient permissions. Required roles: {:?}, User role: {}", config.required_roles, user.role_id)
                    }));
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body());
            }
            // Add user information to request extensions for use in handlers
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(claims);
            req.extensions_mut().insert(token);

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}

// Helper functions for common authorization patterns
pub fn require_admin() -> Authorization {
    Authorization::require_admin()
}

pub fn require_user() -> Authorization {
    Authorization::require_user()
}

pub fn require_roles(roles: Vec<i32>) -> Authorization {
    Authorization::require_roles(roles)
}

pub fn require_any_role(roles: Vec<i32>) -> Authorization {
    Authorization::new(AuthorizationConfig {
        required_roles: roles,
        check_permissions: false,
    })
}

// Extract user from request extensions
pub fn get_current_user(req: &ServiceRequest) -> Option<User> {
    req.extensions().get::<User>().cloned()
}

// Extract claims from request extensions
pub fn get_current_claims(req: &ServiceRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}

// Extract raw token from request extensions
pub fn get_current_token(req: &ServiceRequest) -> Option<String> {
    req.extensions().get::<String>().cloned()
}
