use rbatis::crud::CRUDTable;
use rbatis::DateTimeNative;

#[crud_table(table_name: iorecord_iorecord)]
pub struct IORecord {
    id: i32,
    user_id: String,
    user_name: String,
    pub(crate) occur_time: DateTimeNative,
    device_id: String,
    device_name: String,
    pub(crate) which_library: String,
    is_in: i32,
}