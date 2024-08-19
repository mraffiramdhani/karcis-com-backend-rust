use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    app::{
        auth::models::Role,
        utils::{standard_response::StandardResponse, token_signing::TokenSigning},
    },
    db::DbPool,
};

pub struct HasRole {
    required_role: String, // {{ edit_1 }} Add a field for the required role
}

impl HasRole {
    pub fn new(required_role: String) -> Self {
        // {{ edit_1 }} Add a constructor to initialize required_role
        HasRole { required_role }
    }
}

impl<S, B> Transform<S, ServiceRequest> for HasRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = HasRoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HasRoleMiddleware {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct HasRoleMiddleware<S> {
    service: Rc<S>,
    required_role: String,
}

impl<S, B> Service<ServiceRequest> for HasRoleMiddleware<S>
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
        let service = Rc::clone(&self.service);
        let required_role = self.required_role.clone(); // {{ edit_3 }} Clone the required role

        Box::pin(async move {
            // Validate user role
            let auth_header = req.headers().get("Authorization");
            let mut token = "";
            if let Some(header_value) = auth_header {
                if let Ok(value) = header_value.to_str() {
                    token = value;
                }
            }
            let trimmed_token = token
                .split_whitespace()
                .nth(1)
                .expect("Bearer token not found");
            let token_data = TokenSigning::verify(trimmed_token).expect("Failed to verify token");
            let role_id = token_data.claims.role_id;
            let pool = req.app_data::<Data<DbPool>>().unwrap();
            let mut transaction = pool.begin().await.expect("Failed to initialize DbPool");
            let user_role_string = Role::get_by_id(&mut transaction, &role_id)
                .await
                .expect("Failed to fetch user role string");
            if user_role_string.name != required_role {
                let http_res =
                    HttpResponse::Forbidden().json(StandardResponse::<()>::error(format!(
                        "You don't have access to this endpoint. Required Access Role: {}",
                        required_role.clone().to_string()
                    )));
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body()); // {{ edit_5 }} Return forbidden if role doesn't match
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
