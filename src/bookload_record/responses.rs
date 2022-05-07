use rocket::serde::Serialize;
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct BookListResponse {
    pub id: i32,
    pub name: String,
    pub r#date: NaiveDate
}