use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserFormModel {
    pub email: String,
    pub password: String,
}
