use deadpool_redis::{redis::{AsyncCommands}, Pool, Connection};
use rbatis::Error;
use std::string::String;
use redis::{FromRedisValue, ToRedisArgs};
// use redis::Value::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::http::{Cookie, CookieJar};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::State;
use serde::{Deserialize, Serialize};
use rocket::serde::DeserializeOwned;

pub struct Session {
    pub connect: Connection,
    pub session_id: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut token = request.cookies().get_private("session");

        if token.is_none() {
            token = Some(Cookie::new("session", "123321"));
            request.cookies().add_private(token.clone().unwrap());
            // return Outcome::Failure((Status::Unauthorized, ()));
        }
        let token = token.unwrap().value().to_string();
        return Outcome::Success(
            Session{
                connect: request.rocket().state::<Pool>().unwrap().get().await.unwrap(),
                session_id:token
            }
        );
    }
}

// #[rocket::async_trait]
impl Session {
    pub async fn get<T>(&mut self, key: &str) -> Option<T> where
    T: DeserializeOwned
    {
        let x: Option<String> = self.connect.hget(&self.session_id, key).await.unwrap();
        if let Some(x) = x {
            let a =  Some(serde_json::from_str(&x).unwrap());
            return a;
        }
        return None;
    }

    pub async fn set<T>(&mut self, key: &str, value: T) -> T where
    T: Serialize
    {
        let value_ = serde_json::to_string(&value).unwrap();
        self.connect.hset::<_, _, _, i32>(&self.session_id, key, &value_).await.unwrap();
        value
    }
}