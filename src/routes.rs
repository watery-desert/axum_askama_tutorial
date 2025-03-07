use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    handlers::{
        auth::{
            log_in_handler, log_out_handler, post_login_handler, post_sign_up_hander,
            sign_up_handler,
        },
        public::{home, page_not_found_handler},
        todos::{
            create_todo_handler, delete_todo_handler, post_create_todo_handler, todos_handler,
            toggle_todo_handler,
        },
    },
    middlewares::csp,
    setup_logging,
};

use crate::{
    middlewares::auth::{authenticate, redirect_auth_user, required_authentication},
    models::app::AppState,
};

pub fn router(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .merge(auth_routes())
        .nest_service("/static", server_dir)
        .merge(protected_routes())
        .fallback(page_not_found_handler)
        .layer(middleware::from_fn(authenticate))
        .with_state(app_state)
        .layer(middleware::from_fn(csp::secure_headers))
        .layer(setup_logging!());

    app
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/sign-up", get(sign_up_handler).post(post_sign_up_hander))
        .route("/log-in", get(log_in_handler).post(post_login_handler))
        .layer(middleware::from_fn(redirect_auth_user))
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/log-out", post(log_out_handler))
        .nest("/todos", todo_routes())
        .route_layer(middleware::from_fn(required_authentication))
}

fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/:page", get(todos_handler)) // -> /todos/
        .route(
            "/create", // -> /todos/create
            get(create_todo_handler).post(post_create_todo_handler),
        )
        .route("/toggle/:id", post(toggle_todo_handler))
        .route("/delete/:id", post(delete_todo_handler))
}
