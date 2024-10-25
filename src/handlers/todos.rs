use super::errors::AppError;
use crate::{
    data::todo,
    models::{
        app::{AppState, CurrentUser},
        templates::{CreateTemplate, TodosTemplate},
        todo_form_model::CreateTodoFormModel,
    },
};
use askama::Template;
use axum::{
    extract::State,
    response::{Extension, Html, IntoResponse, Response},
    Form,
};
use tower_sessions::Session;

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

// pub async fn post_create_todo_handler(
//     session: Session,
//     Extension(current_user): Extension<CurrentUser>,
//     State(app_state): State<AppState>,
//     Form(create_todo_form): Form<CreateTodoFormModel>,
// ) -> Result<Response, AppError> {
//     let user_id = current_user.user_id.unwrap();

//     let result = todo::create(&app_state.connection_pool, &create_todo_form.task, &user_id).await;

    

    
// }
