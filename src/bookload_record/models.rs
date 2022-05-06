use rbatis::crud::CRUDTable;
use rbatis::{DateTimeNative, DateNative};

#[crud_table(table_name: bookloadrecord_bookloadrecord)]
pub struct Bookloadrecord {
    id: i32,
    user_id: i32,
    user_name: String,
    reserve_date: DateNative,
    pub(crate) reserve_begin: DateTimeNative,
    reserve_end: DateTimeNative,
    pub dev_name: String,
    pub(crate) which_library: String,
}