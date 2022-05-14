use std::collections::HashMap;
use std::ops::Sub;
use std::string::String;
use std::sync::Arc;

use chrono::{DateTime, Duration, NaiveDate};
use rbatis::crud::CRUD;
use rbatis::DateTimeNative;
use rbatis::rbatis::Rbatis;
use rocket::{Request, State};
use rocket::form::Form;
use rocket::response::{content, status};
use serde::de::Unexpected::Option;

use crate::User;
use crate::utils::guards::IsLogin;

use super::forms::IOQueryForm;
use super::models::IORecord;

#[post("/library-heatmap", data = "<query>")]
pub async fn heatmap(rb: &State<Arc<Rbatis>>, query: Form<IOQueryForm>, userinfo: IsLogin)
                    -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let query = query.into_inner();
    let mut fmt = "%Y-%m-%d";
    if query.cell.eq("month") {
        fmt = "%Y-%m";
    }
    let start = NaiveDate::parse_from_str(&query.start, &fmt).unwrap();
    let end = NaiveDate::parse_from_str(&query.end, &fmt).unwrap();
    let w = rb.new_wrapper()
        .ge("occur_time", start)
        .le("occur_time", end)
        .eq("is_in", 1)
        .eq("which_library", &query.library)
        .eq("user_id", &user.user_id)
        ;
    let mut io_records: Vec<IORecord> = rb
        .fetch_list_by_wrapper(w)
        .await
        .unwrap();
    let mut ans: HashMap<_, _> = io_records
        .iter()
        .map(|x| { (x.occur_time.format(&fmt).to_string(), 0) })
        .collect();
    for i in io_records {
        let mut x = ans.get_mut(&i.occur_time.format(&fmt).to_string()[..]);
        if let Some(cnt) = x {
            *cnt += 1;
        }
    }
    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}

#[post("/library-info")]
pub async fn library_info(rb: &State<Arc<Rbatis>>, userinfo: IsLogin)
                    -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let mut fmt = "%Y年%m月%d日";
    let w = rb.new_wrapper()
        .eq("is_in", true)
        .eq("user_id", "11912823")
        .order_by(true, &["occur_time"])
        ;
    let mut io_records: Vec<IORecord> = rb
        .fetch_list_by_wrapper(w)
        .await
        .unwrap();
    let mut ans = HashMap::new();
    ans.insert("琳恩", 0.to_string());
    ans.insert("涵泳", 0.to_string());
    ans.insert("一丹", 0.to_string());
    if io_records.len() == 0usize {
        return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())));
    }
    let mut yd = 0;
    let mut hy = 0;
    let  mut le = 0;
    ans.insert("first_name", io_records[0].which_library.clone());
    ans.insert("first_time", io_records[0].occur_time.format(&fmt).to_string());
    for i in &io_records {
        match &i.which_library[..] {
            "琳恩" => le += 1,
            "涵泳" => hy += 1,
            "一丹" => yd += 1,
            _ => {}
        }
    }
    ans.insert("琳恩", le.to_string());
    ans.insert("涵泳", hy.to_string());
    ans.insert("一丹", yd.to_string());
    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}
