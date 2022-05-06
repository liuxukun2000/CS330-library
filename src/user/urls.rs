use super::apis::{login, static_info, user_tag, userinfo, test};

pub fn routes() -> Vec<rocket::Route> {
    routes![test, login, userinfo, static_info,user_tag]
}