use super::errors::AppError;
use crate::models::{
    app::CurrentUser,
    templates::{CreateTemplate, TodosTemplate},
};
use askama::Template;
use axum::response::{Extension, Html, IntoResponse, Response};

pub async fn todos_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let html_string = TodosTemplate {
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}

pub async fn create_todo_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let html_string = CreateTemplate {
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}
