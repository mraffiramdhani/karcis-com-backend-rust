use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    web::{self},
    Error, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt};

use crate::{app::auth::models::Token, db::DbPool};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        let auth = req.headers().get(AUTHORIZATION);
        if auth.is_none() {
            let http_res = HttpResponse::Unauthorized().finish();
            let (http_req, _) = req.into_parts();
            let res = ServiceResponse::new(http_req, http_res);
            return (async move { Ok(res.map_into_right_body()) }).boxed_local();
        }

        let token = auth.unwrap().to_str().unwrap().to_string(); // Extract token from header
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Check if the token is revoked
            let mut transaction = req
                .app_data::<web::Data<DbPool>>()
                .unwrap()
                .begin()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; // Convert error
            if Token::is_token_revoked(&mut transaction, &token)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
            {
                let http_res = HttpResponse::Unauthorized().finish();
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body());
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
