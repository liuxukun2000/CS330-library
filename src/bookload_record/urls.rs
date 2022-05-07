use super::apis::{loan_info, loan_list};

pub fn routes() -> Vec<rocket::Route> {
    routes![loan_info, loan_list]
}