use super::{
    errors::AppError,
    functions::{next_page, previous_page, todo_date},
    helpers,
};
use crate::{
    data::todo,
    handle_client_error,
    models::{
        app::{AppState, CurrentUser, FlashStatus},
        templates::{CreateTemplate, TodosTemplate},
        todo_form_model::{CreateTodoFormModel, TodoPageQuery, ToggleTodoFormModel},
    },
};
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    response::{Extension, Html, IntoResponse, Redirect, Response},
    Form,
};
use tower_sessions::Session;

pub async fn todos_handler(
    State(app_state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(page): Path<i32>,
    session: Session,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;
    let user_id = current_user.user_id.unwrap();

    let page_size: i32 = 3;

    let todos = todo::get_all(&app_state.connection_pool, &user_id, &page_size, &page).await?;
    let total_todos = todo::get_total_todos(&app_state.connection_pool).await?;

    let total_pages = (total_todos + page_size - 1) / page_size;

    let html_string = TodosTemplate {
        is_authenticated: current_user.is_authenticated,
        flash_data,
        todos,
        current_page: page,
        total_pages,
        next_page,
        previous_page,
        todo_date,
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

    handle_client_error!(result, &session, Redirect::to("/todos/1").into_response());

    session.insert("flash", "Todo created successfully").await?;
    session
        .insert("flash_status", FlashStatus::Success.to_string())
        .await?;
    Ok(Redirect::to("/todos/1").into_response())
}

pub async fn toggle_todo_handler(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Query(todo_page_query): Query<TodoPageQuery>,
    Form(todo_form): Form<ToggleTodoFormModel>,
) -> Result<Response, AppError> {
    todo::toggle(&app_state.connection_pool, &id, &todo_form.is_done).await?;

    let path = format!("/todos/{}", todo_page_query.page);

    Ok(Redirect::to(&path).into_response())
}

pub async fn delete_todo_handler(
    Path(id): Path<i32>,
    Query(todo_page_query): Query<TodoPageQuery>,
    State(app_state): State<AppState>,
) -> Result<Response, AppError> {
    todo::delete(&app_state.connection_pool, &id).await?;
    let path = format!("/todos/{}", todo_page_query.page);

    Ok(Redirect::to(&path).into_response())
}
