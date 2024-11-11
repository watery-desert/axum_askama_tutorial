use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoFormModel {
    pub task: String,
}

#[derive(Deserialize)]
pub struct ToggleTodoFormModel {
    pub is_done: bool,
}

#[derive(Deserialize)]
pub struct TodoPageQuery {
    pub page: usize,
}
