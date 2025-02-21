use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::middleware::{Middleware, Response};
use jsonwebtoken::{decode, Validation, DecodingKey};
use crate::models::Claims;
use std::env;

pub struct AuthMiddleware;

impl<S, B> Middleware<S> for AuthMiddleware
where
    S: Service,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest, srv: &S) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if let Some(auth_header) = auth_header {
            if let Ok(token) = auth_header.to_str() {
                let token_data = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
                    &Validation::default(),
                );
                if let Ok(token_data) = token_data {
                    req.extensions_mut().insert(token_data.claims);
                }
            }
        }
        Box::pin(async move { srv.call(req).await })
    }
}