use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoFormModel {
    pub task: String,
}