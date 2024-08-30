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
pub struct SignUpTemplate {}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LogInTemplate {}
