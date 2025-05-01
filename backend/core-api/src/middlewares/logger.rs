// /home/inno/elights_jobes-research/backend/core-api/src/middlewares/logger.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Instant;

#[derive(Default, Clone)] // Added Default and Clone
pub struct RequestLogger;

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware {
            service: Rc::new(service),
        })
    }
}

// Middleware implementation
pub struct RequestLoggerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let query = req.uri().query().map_or("", |q| q).to_string(); // Handle optional query
        let peer_addr = req.peer_addr().map(|s| s.to_string()).unwrap_or_else(|| "-".to_string());

        // Log basic request info immediately
        log::info!(
            "--> Incoming Request | Peer: {} | Method: {} | Path: {} | Query: {}",
            peer_addr,
            method,
            path,
            if query.is_empty() { "-" } else { &query }
        );


        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?; // Wait for the response

            let duration = start_time.elapsed();
            let status = res.status();

            // Log response info
            log::info!(
                 "<-- Response Sent | Status: {} | Duration: {:?} | Method: {} | Path: {}",
                 status.as_u16(),
                 duration,
                 method,
                 path,
            );

            Ok(res)
        })
    }
}