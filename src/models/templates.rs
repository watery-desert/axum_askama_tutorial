use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate {}

#[derive(Template)]
#[template(path = "pages/create.html")]
pub struct CreateTemplate {}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
pub struct SignUpTemplate<'a> {
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LogInTemplate {}
