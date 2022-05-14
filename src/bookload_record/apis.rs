use std::collections::{HashMap, HashSet};
// use std::str::pattern::Pattern;
use std::string::String;
use std::sync::Arc;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
// use redis::AsyncCommands;
use rocket::{Request, State};
use rocket::form::{Form, Strict};
use rocket::futures::StreamExt;
use rocket::http::{Cookie, CookieJar};
use rocket::http::ext::IntoCollection;
use rocket::response::{content, status};
use rocket::response::content::Json;
use serde::de::value::StrDeserializer;

use crate::utils::guards::IsLogin;
use crate::book::models::Book;

use super::models::{Bookloadrecord, get_types, get_words};
use super::responses::BookListResponse;
use itertools::Itertools;

static default_words: [&str; 15] = ["大学", "书院", "南方科技", "深圳", "敢为天下先", "青春", "一丹",
    "琳恩", "函泳", "学业", "热爱", "生活", "自强", "求是", "求真"];

#[post("/loan-info")]
pub async fn loan_info(rb: &State<Arc<Rbatis>>, userinfo: IsLogin) -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let loan_info_wrapper = rb
        .new_wrapper()
        .eq("user_id", user.id)
        .order_by(true, &["loan_date"]);
    let loan_info: Vec<Bookloadrecord> = rb
        .fetch_list_by_wrapper(loan_info_wrapper)
        .await
        .unwrap();
    let mut ans = HashMap::new();
    if loan_info.len() == 0 {
        return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
    }
    let book: Book = rb
        .fetch_by_column("id", loan_info[0].book_id)
        .await
        .unwrap();
    ans.insert("name", book.show_title);
    let count_wrapper = rb
        .new_wrapper()
        .eq("book_id", book.id)
        .ne("user_id", user.id)
        .group_by(&["user_id, id"])
        ;
    let count = rb.fetch_list_by_wrapper::<Bookloadrecord>(count_wrapper).await.unwrap();
    ans.insert("count", count.len().to_string());
    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}

#[post("/loan-list")]
pub async fn loan_list(rb: &State<Arc<Rbatis>>, userinfo: IsLogin) -> status::Accepted<Json<String>> {
    let user = userinfo.0;
    let loan_info_wrapper = rb
        .new_wrapper()
        .eq("user_id", user.id)
        // .group_by(&["book_id"])
        .order_by(true, &["loan_date"]);
    let loan_info: Vec<Bookloadrecord> = rb
        .fetch_list_by_wrapper(loan_info_wrapper)
        .await
        .unwrap();
    let mut ans = HashMap::new();
    if loan_info.len() == 0 {
        return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())))
    }
    let books: Vec<Book> = rb
        .fetch_list_by_column(
            "id",
            &(loan_info
                .iter()
                .map(|x|x.book_id)
                .collect::<Vec<i32>>())
        )
        .await
        .unwrap();
    let book_hash: HashMap<_, _> = books
        .iter()
        .map(|x| (x.id, x.show_title.clone()))
        .collect();
    ans.insert(
        "list",
        loan_info
            .iter()
            .enumerate()
            .map(
                |x|
                    BookListResponse
                        {
                            id: x.0 as i32,
                            name: book_hash
                                .get(&x.1.book_id)
                                .unwrap()
                                .clone(),
                            date: x.1.loan_date
                                .format("%Y年%m月%d日")
                                .to_string()
                        }
            )
            .collect::<Vec<BookListResponse>>()
    );
    return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())))
}

#[post("/loan-type")]
pub async fn loan_type(rb: &State<Arc<Rbatis>>, userinfo: IsLogin) -> status::Accepted<Json<String>> {
    // let user = userinfo.0;

    let types: Vec<Book> = get_types(rb, &(userinfo.0.id), &3usize).await.unwrap();
    let mut ans: HashMap<&str, i32> = HashMap::new();
    for i in &types {
         let count = ans.entry(&i.r#type).or_insert(0);
        *count += 1;
    }
    return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())))
}


#[post("/words")]
pub async fn words(rb: &State<Arc<Rbatis>>, userinfo: IsLogin) -> status::Accepted<Json<String>> {
    let mut types: Vec<Book> = get_words(rb, &(userinfo.0.id), &15usize)
        .await
        .unwrap();
    types.iter()
        .update(|x| {
            log::info!("{}", x.keyword.clone());
            x.keyword.strip_prefix("['");
            x.keyword.strip_suffix("']");
            x.writer_keyword.strip_prefix("['");
            x.writer_keyword.strip_suffix("']");
        });
    let mut ans: HashMap<&str, Vec<String>> = HashMap::new();

    let mut words = vec![];
    for i in &types {
        if words.len() > 20 {
            break;
        }
        if i.keyword.len() >= 1 {
            words.push(i.keyword.clone());
        }
        if i.writer_keyword.len() >= 1 {
            words.push(i.writer_keyword.clone());
        }
    }
    let mut types: HashSet<String> = HashSet::from_iter(words.into_iter());
    let mut words:Vec<String> = types.iter().map(|x| x.clone()).collect();
    let mut num = 0;
    while words.len() < 15 {
        words.push(default_words[num].to_string());
        num += 1;
    }
    ans.insert("list", words);
    return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())))
}