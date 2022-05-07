use std::collections::HashMap;
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

use super::models::Bookloadrecord;
use super::responses::BookListResponse;
use itertools::Itertools;


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
        .group_by(&["user_id"])
        ;
    let count = rb.fetch_list_by_wrapper::<Bookloadrecord>(count_wrapper).await.unwrap();
    ans.insert("count", count.len().to_string());
    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}

#[post("/loan-list")]
pub async fn loan_list(rb: &State<Arc<Rbatis>>, userinfo: IsLogin) -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let loan_info_wrapper = rb
        .new_wrapper()
        .eq("user_id", user.id)
        .group_by(&["book_id"])
        .order_by(true, &["loan_date"]);
    let loan_info: Vec<Bookloadrecord> = rb
        .fetch_list_by_wrapper(loan_info_wrapper)
        .await
        .unwrap();
    let mut ans = HashMap::new();
    if loan_info.len() == 0 {
        return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
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
                            date: x.1.loan_date.inner
                        }
            )
            .collect::<Vec<BookListResponse>>()
    );
    return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}