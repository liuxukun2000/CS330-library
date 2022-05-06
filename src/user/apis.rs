use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rocket::{Request, State};
use rocket::form::{Form, Strict};
use rocket::http::{Cookie, CookieJar};
use rocket::response::{content, status};
use rocket::response::content::Json;
use serde::de::value::StrDeserializer;

use crate::utils::guards::IsLogin;

use super::forms;
use super::models::{StaticInfo, User};
use super::responses::StaticResponse;

#[post("/login", data = "<userinfo>")]
pub async fn login(rb: &State<Arc<Rbatis>>, cookies: &CookieJar<'_>, userinfo: Form<forms::UserLoginForm>)
                   -> status::Accepted<content::Json<String>> {
    let userinfo = userinfo.into_inner();
    let result: Option<User> = rb.fetch_by_column("user_id", &userinfo.username)
        .await.unwrap();
    let x = serde_json::to_string(&result.unwrap()).unwrap();
    cookies.add_private(Cookie::new("token", x.clone()));
    status::Accepted(Some(content::Json(x)))
}

#[post("/userinfo")]
pub async fn userinfo(userinfo: IsLogin) -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let info = serde_json::to_string(&user).unwrap();
    status::Accepted(Some(content::Json(info)))
}

#[post("/static-info/<type>")]
pub async fn static_info(rb: &State<Arc<Rbatis>>, r#type: String, userinfo: IsLogin) -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let info: Option<StaticInfo> = rb.fetch_by_column("user_id", user.id).await.unwrap();
    let mut ans = StaticResponse { count: 0, length: 0, r#type: 0, percentage: 0.0 };
    if info.is_none() {
        return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())));
    }
    let info = info.unwrap();
    match &r#type[..] {
        "book" => {
            ans.count = info.loadbook_count;
            ans.r#type = info.loadbook_type;
            if let Some(p) = info.loadbook_percentage {
                ans.percentage = p;
            }
        }
        "library" => {
            ans.count = info.librarytime_count;
            ans.length = info.librarytime_length;
            if let Some(p) = info.librarytime_percentage {
                ans.percentage = p;
            }
        }
        "discussion-room" => {
            ans.count = info.discussroom_count;
            ans.length = info.discussroom_length;
            if let Some(p) = info.discussroom_percentage {
                ans.percentage = p;
            }
        }
        _ => {}
    };
    return status::Accepted(Some(Json(serde_json::to_string(&ans).unwrap())));
}

#[post("/user-tag")]
pub async fn user_tag(rb: &State<Arc<Rbatis>>, userinfo: IsLogin)
                      -> status::Accepted<content::Json<String>> {
    let user = userinfo.0;
    let info: Option<StaticInfo> = rb.fetch_by_column("user_id", user.id).await.unwrap();
    let mut ans = HashMap::new();
    if info.is_none() {
        ans.insert("tag", "神秘人".to_string());
        ans.insert("comment", "您的行踪过于神秘，还没有进入过图书馆。".to_string());
        return status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())));
    }
    let info = info.unwrap();
    if info.librarytime_length == 0 {
        ans.insert("tag", "神秘人".to_string());
        ans.insert("comment", "您的行踪过于神秘，还没有进入过图书馆。".to_string());
    } else if info.librarytime_percentage.unwrap() >= 70f32 {
        ans.insert("tag", "勤学者".to_string());
        ans.insert("comment", format!("您一共在图书馆学习了{}h，超过了{}%的读者，努力就会有回报，请继续加油！
", info.librarytime_length / 60, info.librarytime_percentage.unwrap()));
    } else if info.discussroom_count > 5 {
        ans.insert("tag", "思辩者 ".to_string());
        ans.insert("comment", format!("你一共预约了{}次讨论间，超过了{}%的读者，思想在辩论中捧出火花，祝您遇到的问题都能迎刃而解！",
                                      info.discussroom_count, info.discussroom_percentage.unwrap()));
    } else if info.loadbook_type > 5 {
        ans.insert("tag", "博览者".to_string());
        ans.insert("comment", format!("您在图书馆阅读了{}种书籍，超过了{}%的读者，令人膜拜！",
                                      info.loadbook_type, info.loadbook_percentage.unwrap()));
    } else {
        ans.insert("tag", "思索者".to_string());
        ans.insert("comment", "善于思考，博学多识，您一定是大家的学习榜样！".to_string());
    }

    status::Accepted(Some(content::Json(serde_json::to_string(&ans).unwrap())))
}

#[get("/test")]
pub async fn test() -> &'static str {
    "hello"
}