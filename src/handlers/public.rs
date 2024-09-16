use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use super::errors::AppError;

pub async fn home() -> Result<Response, AppError> {
    let html_string = HomeTemplate {}.render().unwrap();

    Ok(Html(html_string).into_response())
}
