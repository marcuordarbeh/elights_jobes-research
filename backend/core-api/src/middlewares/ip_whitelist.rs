// /home/inno/elights_jobes-research/backend/core-api/src/middlewares/ip_whitelist.rs

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::{
    future::Future,
    net::IpAddr,
    pin::Pin,
    rc::Rc, // Or Arc if service needs to be Send + Sync
    task::{Context, Poll},
    collections::HashSet, // Efficient IP checking
    str::FromStr,
};

// Configuration struct for the middleware
#[derive(Clone)]
pub struct IpWhitelist {
    allowed_ips: Rc<HashSet<IpAddr>>, // Use Rc for single-threaded Actix actors
}

impl IpWhitelist {
    pub fn new(allowed_ips_str: Vec<String>) -> Self {
        let allowed_ips = allowed_ips_str
            .into_iter()
            .filter_map(|ip_str| IpAddr::from_str(&ip_str).ok())
            .collect::<HashSet<IpAddr>>();
        log::info!("IP Whitelist initialized with: {:?}", allowed_ips);
        Self {
            allowed_ips: Rc::new(allowed_ips),
        }
    }
}

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for IpWhitelist
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = IpWhitelistMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(IpWhitelistMiddleware {
            service: Rc::new(service),
            allowed_ips: Rc::clone(&self.allowed_ips),
        })
    }
}

// Middleware implementation
pub struct IpWhitelistMiddleware<S> {
    service: Rc<S>,
    allowed_ips: Rc<HashSet<IpAddr>>,
}

impl<S, B> Service<ServiceRequest> for IpWhitelistMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Use Actix 4's forward_ready! macro
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_allowed = match req.peer_addr() {
            Some(socket_addr) => self.allowed_ips.contains(&socket_addr.ip()),
            None => {
                log::warn!("Could not get peer address for IP whitelist check.");
                false // Or handle based on policy (e.g., deny if address unknown)
            }
        };

        if is_allowed {
            log::debug!("IP {:?} allowed.", req.peer_addr().map(|s| s.ip()));
            let fut = self.service.call(req);
            Box::pin(async move {
                 let res = fut.await?;
                 Ok(res)
            })
        } else {
            log::warn!("IP {:?} blocked by whitelist.", req.peer_addr().map(|s| s.ip()));
            Box::pin(async move {
                // Create the forbidden response directly
                let response = HttpResponse::Forbidden()
                    .body("IP address not allowed")
                    .map_into_right_body(); // Convert body type

                // req.into_response(...) is used to construct the response
                // while consuming the request properly in middleware chains.
                Ok(req.into_response(response))
            })
        }
    }
}