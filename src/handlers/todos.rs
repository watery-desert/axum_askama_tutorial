use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{TodosTemplate, CreateTemplate};
use askama::Template;
use super::errors::AppError;

pub async fn todos_handler() -> Result<Response, AppError> {
    let html_string = TodosTemplate {}.render().unwrap();

    Ok(Html(html_string).into_response())
}

pub async fn create_todo_handler() -> Result<Response, AppError> {
    let html_string = CreateTemplate {}.render().unwrap();

    Ok(Html(html_string).into_response())
}
