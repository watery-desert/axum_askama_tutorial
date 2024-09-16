use axum_askama_tutorial::{init, routes, models::app::AppState};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind addr");

    init::logging();

    let pg_pool = init::database_connection().await;

    let app_state = AppState {
        connection_pool: pg_pool
    };

    tracing::info!("Server is starting...");

    tracing::info!("Listening at {}", addr);

    let app = routes::router(app_state);

    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
