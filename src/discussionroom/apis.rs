use std::collections::HashMap;
use std::ops::Sub;
use std::string::String;
use std::sync::Arc;

use chrono::{DateTime, Duration, NaiveDate};
use rbatis::crud::CRUD;
use rbatis::{DateTimeNative, DateNative};
use rbatis::rbatis::Rbatis;
use rocket::{Request, State};
use rocket::form::Form;
use rocket::response::{content, status};
use serde::de::Unexpected::Option;

use crate::User;
use crate::utils::guards::IsLogin;

use super::models::DiscussionRoom;


#[post("/discussionroom-info")]
pub async fn discussionroom_info(rb: &State<Arc<Rbatis>>, userinfo: IsLogin)
                    -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let mut fmt = "%Y年%m月%d日";
    let w = rb.new_wrapper()
        .eq("user_id", &user.user_id)
        .order_by(true, &["reserve_begin"])
        ;
    let mut records: Vec<DiscussionRoom> = rb
        .fetch_list_by_wrapper(w)
        .await
        .unwrap();
    let mut ans = HashMap::new();
    if records.len() == 0usize {
        return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())));
    }
    ans.insert("first_time", records[0].reserve_begin.format(&fmt).to_string());
    let mut count: HashMap<_, _> = records
        .iter()
        .map(|x| {(x.dev_name.clone(), 0)})
        .collect();
    let mut max_cnt = 0;
    for i in records {
        let mut cnt = count.get_mut(&i.dev_name[..]).unwrap();
        *cnt += 1;
        if *cnt > max_cnt {
            max_cnt = *cnt;
            ans.insert("room", i.dev_name.clone());
            ans.insert("library", i.which_library.clone());
        }
    }
    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}