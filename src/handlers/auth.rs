use crate::{
    handle_client_error,
    models::{
        app::CurrentUser,
        templates::{LogInTemplate, SignUpTemplate},
        user_form_model::AuthFormModel,
    },
};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Extension, Form,
};
use tower_sessions::Session;
use validator::Validate;

use super::{errors::AppError, helpers};

use crate::{
    data::user,
    models::app::{AppState, FlashStatus},
};

pub async fn sign_up_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    if current_user.is_authenticated {
        return Ok(Redirect::to("/todos/1").into_response());
    }

    let flash_data = helpers::get_flash(&session).await?;

    let html_string = SignUpTemplate {
        is_authenticated: current_user.is_authenticated,
        email: "",
        email_error: "",
        password_error: "",
        flash_data,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}

pub async fn post_sign_up_hander(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
    State(app_state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let result = user::create_user(
                &app_state.connection_pool,
                &user_form.email,
                &user_form.password,
            )
            .await;

            handle_client_error!(result, &session, Redirect::to("/sign-up").into_response());

            helpers::set_flash(
                &session,
                "Account created successfully, now login".to_string(),
                FlashStatus::Success.to_string(),
            )
            .await?;

            Ok(Redirect::to("/log-in").into_response())
        }
        Err(errs) => {
            let errs = errs.to_string();

            let mut email_error = String::new();
            let mut password_error = String::new();

            helpers::extract_error(&errs, |field, message| {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message
                }
            });

            let flash_data = helpers::get_flash(&session).await?;

            let html_string = SignUpTemplate {
                is_authenticated: current_user.is_authenticated,
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                flash_data,
            }
            .render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}

pub async fn log_in_handler(
    session: Session,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let flash_data = helpers::get_flash(&session).await?;

    let html_string = LogInTemplate {
        is_authenticated: current_user.is_authenticated,
        email: "",
        email_error: "",
        password_error: "",
        flash_data,
    }
    .render()?;

    Ok(Html(html_string).into_response())
}

pub async fn post_login_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(app_state): State<AppState>,
    session: Session,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
    match user_form.validate() {
        Ok(_) => {
            let user_id = user::authenticate_user(
                &app_state.connection_pool,
                &user_form.email,
                &user_form.password,
            )
            .await;

            let user_id =
                handle_client_error!(user_id, &session, Redirect::to("/log-in").into_response());

            session.insert("authenticated_user_id", user_id).await?;
            Ok(Redirect::to("/todos/1").into_response())
        }
        Err(errs) => {
            let errs = errs.to_string();

            let mut email_error = String::new();
            let mut password_error = String::new();

            helpers::extract_error(&errs, |field, message| {
                if field == "email" {
                    email_error = message;
                } else if field == "password" {
                    password_error = message
                }
            });

            let flash_data = helpers::get_flash(&session).await?;

            let html_string = LogInTemplate {
                is_authenticated: current_user.is_authenticated,
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                flash_data,
            }
            .render()?;

            let response = Html(html_string).into_response();

            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }
}

pub async fn log_out_handler(session: Session) -> Result<Response, AppError> {
    session.remove::<i32>("authenticated_user_id").await?;

    Ok(Redirect::to("/").into_response())
}
