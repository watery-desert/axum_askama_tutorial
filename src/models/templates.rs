use super::app::FlashData;
use crate::data::todo::Todo;
use askama::Template;
use chrono::Utc;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate {
    pub is_authenticated: bool,
    pub flash_data: FlashData,
    pub todos: Vec<Todo>,
    pub current_page: i32,
    pub total_pages: i32,
    pub next_page: fn(i32) -> i32,
    pub previous_page: fn(i32) -> i32,
    pub todo_date: fn(chrono::DateTime<Utc>) -> String,
}

#[derive(Template)]
#[template(path = "pages/create.html")]
pub struct CreateTemplate {
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
pub struct SignUpTemplate<'a> {
    pub is_authenticated: bool,
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
    pub flash_data: FlashData,
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LogInTemplate<'a> {
    pub is_authenticated: bool,
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
    pub flash_data: FlashData,
}

#[derive(Template)]
#[template(path = "pages/server-error.html")]
pub struct ServerErrorTemplate {
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/not-found.html")]
pub struct PageNotFoundTemplate {
    pub is_authenticated: bool,
}
