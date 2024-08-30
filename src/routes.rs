use axum::{
    body::Body,
    http::{Request, Response},
    routing::get,
    Router,
};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, services::ServeDir, trace::TraceLayer};
use tracing::Span;

use crate::handlers::{
    auth::{log_in_handler, sign_up_handler, post_sign_up_hander},
    public::home,
    todos::{create_todo_handler, todos_handler},
};

pub fn router() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo_handler))
        .route("/todos", get(todos_handler))
        .route("/sign-up", get(sign_up_handler).post(post_sign_up_hander))
        .route("/log-in", get(log_in_handler))
        .nest_service("/static", server_dir)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        );

    app
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "Request started: method {} path {}",
        request.method(),
        request.uri().path()
    )
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed: {:?} after {:?}", error, latency)
}
