use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub connection_pool: PgPool,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub is_authenticated: bool,
    pub user_id: Option<i32>,
}