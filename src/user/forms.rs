use rocket::form::Form;
use rocket::form::FromForm;
use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(FromForm, Deserialize, Serialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}