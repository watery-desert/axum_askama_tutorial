use axum::{
    body::Body,
    http::{Request, Response},
    middleware,
    routing::get,
    Router,
};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, services::ServeDir, trace::TraceLayer};
use tracing::Span;

use crate::handlers::{
    auth::{log_in_handler, post_login_handler, post_sign_up_hander, sign_up_handler},
    public::home,
    todos::{create_todo_handler, todos_handler},
};

use crate::{
    middlewares::{authenticate, required_authentication},
    models::app::AppState,
};

pub fn router(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/sign-up", get(sign_up_handler).post(post_sign_up_hander))
        .route("/log-in", get(log_in_handler).post(post_login_handler))
        .nest_service("/static", server_dir)
        .merge(protected_routes())
        .layer(middleware::from_fn(authenticate))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        );

    app
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", get(create_todo_handler))
        .route("/todos", get(todos_handler))
        .route_layer(middleware::from_fn(required_authentication))
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "-> Request started: method {} path {}",
        request.method(),
        request.uri().path()
    )
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency)
}
