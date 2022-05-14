use super::apis::{loan_info, loan_list, loan_type, words};

pub fn routes() -> Vec<rocket::Route> {
    routes![loan_info, loan_list, loan_type, words]
}