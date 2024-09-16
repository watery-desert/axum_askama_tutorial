use askama::Template;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use axum::response::Html;

use thiserror::Error;

use crate::{data::errors::DataError, models::templates};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AppError::Database(e) => server_error(e.to_string()),
            AppError::Template(e) => server_error(e.to_string()),
        }; 

        (status, response).into_response()

    }
}

fn server_error(e: String) -> (StatusCode, Response<Body>) {
    tracing::error!("Server error: {}", e);

    let html_string = templates::ServerErrorTemplate {}.render().unwrap();

    // match html_string {
    //     Ok(html) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Html(html).into_response(),
    //     ),
    //     Err(e) => {
    //         tracing::error!("Server error: {}", e.to_string());
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Internal server error, please contact me.".into_response(),
    //         )
    //     }
    // }

    (StatusCode::INTERNAL_SERVER_ERROR, Html(html_string).into_response())
}