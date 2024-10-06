use sqlx::PgPool;
use std::fmt::Display;

#[derive(Clone)]
pub struct AppState {
    pub connection_pool: PgPool,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub is_authenticated: bool,
    pub user_id: Option<i32>,
}

pub struct FlashData {
    pub flash: String,
    pub flash_status: String,
}

pub enum FlashStatus {
    Error,
    Info,
    Success,
}

impl Display for FlashStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flash_string = match self {
            FlashStatus::Error => "error-flash",
            FlashStatus::Info => "info-flash",
            FlashStatus::Success => "success-flash",
        };

        write!(f, "{}", flash_string)
    }
}
