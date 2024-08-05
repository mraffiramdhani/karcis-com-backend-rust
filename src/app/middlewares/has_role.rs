use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::Error,
    HttpResponse,
};
use futures_util::future::LocalBoxFuture;

pub struct HasRole {
    required_role: String, // {{ edit_1 }} Add a field for the required role
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
            if !req.headers().contains_key("user-role")
                || req.headers().get("user-role").unwrap() != required_role.as_str()
            {
                let http_res = HttpResponse::Forbidden().finish();
                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);
                return Ok(res.map_into_right_body()); // {{ edit_5 }} Return forbidden if role doesn't match
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
