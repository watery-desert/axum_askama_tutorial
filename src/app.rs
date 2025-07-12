use crate::{handlers::public::get_health_check, init, models::app::AppState, routes::router};
use axum::{routing::get, Router};

pub async fn build_app() -> Router {
    let pg_pool = init::database_connection().await;
    let session_layer = init::session(pg_pool.clone()).await;

    let app_state = AppState {
        connection_pool: pg_pool,
    };

    router(app_state)
        .route("/health_check", get(get_health_check))
        .layer(session_layer)
}
