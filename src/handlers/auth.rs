use crate::models::{
    templates::{LogInTemplate, SignUpTemplate},
    user_form_model::AuthForm,
};
use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use validator::Validate;

pub async fn sign_up_handler() -> Response {
    let html_string = SignUpTemplate {}.render().unwrap();

    Html(html_string).into_response()
}

pub async fn post_sign_up_hander(Form(signup_from): Form<AuthForm>) -> Response {
    match signup_from.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(err) => {
            tracing::info!("{}", err);

            Redirect::to("/").into_response()
        }
    };

    Redirect::to("/").into_response()
}

pub async fn log_in_handler() -> Response {
    let html_string = LogInTemplate {}.render().unwrap();

    Html(html_string).into_response()
}
