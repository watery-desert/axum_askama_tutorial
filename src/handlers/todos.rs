use super::{errors::AppError, helpers};
use crate::{
    data::todo,
    handle_client_error,
    models::{
        app::{AppState, CurrentUser, FlashStatus},
        templates::{CreateTemplate, TodosTemplate},
        todo_form_model::CreateTodoFormModel,
    },
};
use askama::Template;
use axum::{
    extract::State,
    response::{Extension, Html, IntoResponse, Redirect, Response},
    Form,
};
use tower_sessions::Session;

pub async fn todos_handler(
    State(app_state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;
    let user_id = current_user.user_id.unwrap();

    let todos = todo::get_all(&app_state.connection_pool, &user_id).await?;

    let html_string = TodosTemplate {
        is_authenticated: current_user.is_authenticated,
        flash_data, 
        todos
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

pub async fn post_create_todo_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
    State(app_state): State<AppState>,
    Form(create_todo_form): Form<CreateTodoFormModel>,
) -> Result<Response, AppError> {
    let user_id = current_user.user_id.unwrap();

    let result = todo::create(&app_state.connection_pool, &create_todo_form.task, &user_id).await;

    handle_client_error!(result, &session, Redirect::to("/todos").into_response());

    session.insert("flash", "Todo created successfully").await?;
    session
        .insert("flash_status", FlashStatus::Success.to_string())
        .await?;
    Ok(Redirect::to("/todos").into_response())
}
