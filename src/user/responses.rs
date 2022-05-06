use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct StaticResponse {
    pub count: i32,
    pub length: i32,
    pub r#type: i32,
    pub percentage: f32,
}