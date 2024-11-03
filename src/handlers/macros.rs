use axum::response::Response;
use tower_sessions::Session;

use crate::{data::errors::DataError, models::app::FlashStatus};

use super::{errors::AppError, helpers};

#[macro_export]
macro_rules! handle_client_error {
    ($result: expr, $session:expr, $response:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => return $crate::handlers::macros::handle_err(err, $session, $response).await,
        }
    };
}

pub async fn handle_err(
    err: DataError,
    session: &Session,
    response: Response,
) -> Result<Response, AppError> {
    if let DataError::FailedQuery(e) = err {
        helpers::set_flash(&session, e, FlashStatus::Error.to_string()).await?;

        return Ok(response);
    } else {
        Err(err.into())
    }
}
