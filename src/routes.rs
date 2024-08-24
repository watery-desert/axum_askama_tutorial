use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::handlers::{
    auth::{log_in_handler, sign_up_handler},
    public::home,
    todos::{create_todo_handler, todos_handler},
};

pub fn router() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create_todo_handler))
        .route("/todos", get(todos_handler))
        .route("/sign-up", get(sign_up_handler))
        .route("/log-in", get(log_in_handler))
        .nest_service("/static", server_dir);

    app
}
