use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;

use axum::{
    extract::Request,
    http::header::CACHE_CONTROL,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Extension,
};

use tower_sessions::Session;

pub async fn authenticate(
    session: Session,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user_id = session.get::<i32>("authenticated_user_id").await?;

    let mut current_user = CurrentUser {
        is_authenticated: false,
        user_id: None,
    };

    if let Some(id) = user_id {
        current_user.is_authenticated = true;
        current_user.user_id = Some(id);

        req.extensions_mut().insert(current_user);

        Ok(next.run(req).await)
    } else {
        req.extensions_mut().insert(current_user);

        Ok(next.run(req).await)
    }
}

pub async fn required_authentication(
    Extension(current_user): Extension<CurrentUser>,
    req: Request,
    next: Next,
) -> Response {
    if !current_user.is_authenticated {
        return Redirect::to("/log-in").into_response();
    }

    let mut res = next.run(req).await;

    res.headers_mut()
        .insert(CACHE_CONTROL, "no-store".parse().unwrap());

    res
}

pub async fn redirect_auth_user(
    Extension(current_user): Extension<CurrentUser>,
    req: Request,
    next: Next,
) -> Response {
    if current_user.is_authenticated {
        return Redirect::to("/todos").into_response();
    }

    next.run(req).await
}
