use super::apis::{discussionroom_info};

pub fn routes() -> Vec<rocket::Route> {
    routes![discussionroom_info]
}