// use std::net::SocketAddr;
// use axum::extract::ConnectInfo;

use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, Response},
};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Span;

#[macro_export]
macro_rules! setup_logging {
    () => {
        TraceLayer::new_for_http()
            .make_span_with($crate::middlewares::logging::make_span)
            .on_request($crate::middlewares::logging::on_request)
            .on_response($crate::middlewares::logging::on_response)
            .on_failure($crate::middlewares::logging::on_failure)
    };
}

// fn get_ip(request: &Request<axum::body::Body>) -> String {
//     if let Some(ConnectInfo(addr)) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
//         return addr.ip().to_string();
//     }

//     String::from("unknown")
// }

pub fn make_span(_req: &Request<axum::body::Body>) -> Span {
    tracing::info_span!("http-request")
}

pub fn on_request(request: &Request<Body>, _: &Span) {
    // let ip = get_ip(&request);

    // tracing::info!(
    //     "-> Request started: method {} path {} ip {}",
    //     request.method(),
    //     request.uri().path(),
    //     ip
    // )

    tracing::info!(
        "-> Request started: method {} path {}",
        request.method(),
        request.uri().path()
    )
}

pub fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}

pub fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency)
}
