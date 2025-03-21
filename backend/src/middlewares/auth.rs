use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, MessageBody},
    Error,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;
use std::env;

pub struct AuthMiddleware;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware { service })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &Validation::default(),
                ) {
                    req.extensions_mut().insert(token_data.claims);
                }
            }
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
