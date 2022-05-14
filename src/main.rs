#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;


mod user;
mod utils;
mod iorecord;
mod discussionroom;
mod bookload_record;
mod book;
mod plugin;

use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::fairing::AdHoc;

use crate::iorecord::urls as iorecord_urls;
use crate::user::models::User;
use crate::user::urls as user_urls;
use crate::discussionroom::urls as dis_urls;
use crate::bookload_record::urls as book_urls;

use deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime, Pool};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;
use std::error::Error;


async fn check_redis(pool: &Pool) {
    if pool.get().await.is_err() {
        panic!("Redis Server Error!");
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fast_log::init(fast_log::config::Config::new().console());
    log::info!("linking database...");
    let rb = Rbatis::new();



    let rb = Arc::new(rb);
    let mut cfg = Config::from_url("redis://127.0.0.1/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    check_redis(&pool).await;
    let allowed_origins = AllowedOrigins::some_exact(&["https://172.18.24.158"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount("/api", user_urls::routes())
        .mount("/api", iorecord_urls::routes())
        .mount("/api", dis_urls::routes())
        .mount("/api", book_urls::routes())
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        // .attach(cors)
        .manage(pool)
        .launch()
        .await;

    Ok(())
}