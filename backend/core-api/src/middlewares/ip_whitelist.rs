use actix_ip_filter::IPFilter;
use actix_web::{dev::ServiceRequest, Error, HttpResponse, Result};

pub fn ip_whitelist_middleware(allowed: Vec<String>) -> IPFilter {
    IPFilter::new(allowed.into_iter().map(|ip| ip.parse().unwrap()).collect())
        .with_blocked_response(|_req: &ServiceRequest| {
            Ok(HttpResponse::Forbidden().body("IP not allowed"))
        })
}
