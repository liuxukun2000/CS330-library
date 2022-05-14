use rbatis::crud::CRUDTable;

#[crud_table(table_name: user_user)]
pub struct User {
    pub(crate) id: i64,
    pub user_id: String,
    user_name: String,
    reading: bool,
    student_type: String,
    department_id: String,
    department_name: String,
    special_id: String,
    special_name: String,
    sex: bool,
    college_id: Option<String>,
    college_name: Option<String>,
    grade_year: Option<i32>,
    email: String,
}

#[crud_table(table_name: "user_statistics_data_userstatisticsdata")]
pub struct StaticInfo {
    id: i64,
    pub user_id: i32,
    pub discussroom_count: i32,
    pub discussroom_length: i32,
    pub discussroom_percentage: Option<f32>,
    pub loadbook_count: i32,
    pub loadbook_type: i32,
    pub loadbook_percentage: Option<f32>,
    pub librarytime_count: i32,
    pub librarytime_length: i32,
    pub librarytime_percentage: Option<f32>,
}