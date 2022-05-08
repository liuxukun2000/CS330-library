#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

mod user;
mod utils;
mod iorecord;
mod discussionroom;
mod bookload_record;
mod book;

use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::fairing::AdHoc;

use crate::iorecord::urls as iorecord_urls;
use crate::user::models::User;
use crate::user::urls as user_urls;
use crate::discussionroom::urls as dis_urls;
use crate::bookload_record::urls as book_urls;

use deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime, Pool};

async fn check_redis(pool: &Pool) {
    if pool.get().await.is_err() {
        panic!("Redis Server Error!");
    }
}

#[rocket::main]
async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    log::info!("linking database...");
    let rb = Rbatis::new();

    rb.link("sqlite:///home/satan/library/db.sqlite3").await.unwrap();

    let rb = Arc::new(rb);
    let mut cfg = Config::from_url("redis://127.0.0.1/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    check_redis(&pool).await;
    rocket::build()
        .mount("/api", user_urls::routes())
        .mount("/api", iorecord_urls::routes())
        .mount("/api", dis_urls::routes())
        .mount("/api", book_urls::routes())
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .manage(pool)
        .launch()
        .await;
}