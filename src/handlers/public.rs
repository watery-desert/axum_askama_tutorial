use super::errors::AppError;
use crate::models::{app::CurrentUser, templates::HomeTemplate};
use askama::Template;
use axum::response::{Extension, Html, IntoResponse, Response};

pub async fn home(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html_string = HomeTemplate {
        is_authenticated: current_user.is_authenticated,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}
