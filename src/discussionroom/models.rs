use rbatis::crud::CRUDTable;
use rbatis::{DateTimeNative, DateNative, DateTimeUtc, DateUtc};

#[crud_table(table_name: discussionroom_discussionroom)]
pub struct DiscussionRoom {
    id: i32,
    user_id: String,
    user_name: String,
    reserve_date: DateUtc,
    pub(crate) reserve_begin: DateTimeUtc,
    reserve_end: DateTimeUtc,
    pub dev_name: String,
    pub(crate) which_library: String,
}