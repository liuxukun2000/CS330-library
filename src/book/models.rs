use rbatis::crud::CRUDTable;
use rbatis::{DateTimeNative, DateNative};

#[crud_table(table_name: "book_book")]
pub struct Book {
    pub id: i32,
    ISBN: String,
    publisher: String,
    permanent_call_number: String,
    title: String,
    new_title: String,
    pub show_title: String,
    writer: String,
    keyword: String,
    writer_keyword: String,
    r#type: String
}

