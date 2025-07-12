use std::net::SocketAddr;

use axum_askama_tutorial::app::build_app;
use axum_askama_tutorial::init;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind addr");

    init::logging();

    // let pg_pool = init::database_connection().await;

    // let session_layer = init::session(pg_pool.clone()).await;

    // let app_state = AppState {
    //     connection_pool: pg_pool,
    // };

    tracing::info!("Server is starting...");

    tracing::info!("Listening at {}", addr);

    // let app = routes::router(app_state)
    //     .route("/health_check", get(get_health_check))
    //     .layer(session_layer);

    let app = build_app().await;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start the server");
}
