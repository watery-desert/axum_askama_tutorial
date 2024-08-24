use axum_askama_tutorial::routes;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let app = routes::router();

    axum::serve(listener, app).await.unwrap();
}
