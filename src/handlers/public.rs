use super::errors::AppError;
use crate::models::{
    app::CurrentUser,
    templates::{HomeTemplate, PageNotFoundTemplate},
};
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Extension, Html, IntoResponse, Response};

pub async fn home(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html_string = HomeTemplate {
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}

pub async fn page_not_found_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let html_string = PageNotFoundTemplate {
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok((StatusCode::NOT_FOUND, Html(html_string)).into_response())
}

pub async fn get_health_check() -> Response {
    StatusCode::OK.into_response()
}
