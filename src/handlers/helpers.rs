use super::errors::AppError;
use crate::models::app::FlashData;
use axum::{http::{header, StatusCode}, response::{IntoResponse, Response}};
use rust_embed::Embed;
use tower_sessions::Session;

pub fn extract_error<F>(input: &str, mut f: F)
where
    F: FnMut(String, String),
{
    let lines = input.lines();

    lines.for_each(|line| {
        // email: invalid email
        if let Some((first, second)) = line.split_once(": ") {
            f(first.to_string(), second.to_string());
        };
    });
}

pub async fn get_flash(session: &Session) -> Result<FlashData, AppError> {
    let flash = session
        .remove::<String>("flash")
        .await?
        .unwrap_or("".to_string());

    let flash_status = session
        .remove::<String>("flash_status")
        .await?
        .unwrap_or("".to_string());

    Ok(FlashData {
        flash,
        flash_status,
    })
}

pub async fn set_flash(session: &Session, message: String, status: String) -> Result<(), AppError> {
    session.insert("flash", message).await?;
    session.insert("flash_status", status).await?;

    Ok(())
}


#[derive(Embed)]
#[folder = "static/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}