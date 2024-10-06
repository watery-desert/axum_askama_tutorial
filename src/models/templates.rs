use super::app::FlashData;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate {
    pub is_authenticated: bool,
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
