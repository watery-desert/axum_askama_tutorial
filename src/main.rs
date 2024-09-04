use axum_askama_tutorial::{routes, init};

#[tokio::main]
async fn main() {


    let addr = "127.0.0.1:8000";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind addr");

    init::logging();

    // init::database_connection().await;

    tracing::info!("Server is starting...");

    tracing::info!("Listening at {}", addr);

    let app = routes::router();

    axum::serve(listener, app).await.expect("Failed to start the server");

}
