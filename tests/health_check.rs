use axum::http::{self, StatusCode};
use axum_askama_tutorial::app;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let app = app::build_app().await;

    let response = app
        .oneshot(
            http::Request::builder()
                .uri("/health_check")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
