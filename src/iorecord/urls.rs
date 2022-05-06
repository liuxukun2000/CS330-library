use super::apis::{heatmap,library_info};

pub fn routes() -> Vec<rocket::Route> {
    routes![heatmap,library_info]
}