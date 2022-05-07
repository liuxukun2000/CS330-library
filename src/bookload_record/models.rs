use rbatis::crud::CRUDTable;
use rbatis::{DateTimeNative, DateNative};

#[crud_table(table_name: bookloadrecord_bookloadrecord)]
pub struct Bookloadrecord {
    id: i32,
    user_id: i32,
    pub book_id: i32,
    pub(crate) which_library: String,
    pub loan_date: DateNative,
    return_date: Option<DateNative>,
    patron_group: String,
    barcode: String,
    loans: i32
}