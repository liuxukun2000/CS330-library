#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

mod user;
mod utils;
mod iorecord;
mod discussionroom;
mod bookload_record;

use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::fairing::AdHoc;

use crate::iorecord::urls as iorecord_urls;
use crate::user::models::User;
use crate::user::urls as user_urls;
use crate::discussionroom::urls as dis_urls;



#[rocket::main]
async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    log::info!("linking database...");
    let rb = Rbatis::new();

    rb.link("sqlite:///home/satan/library/db.sqlite3").await.unwrap();

    let rb = Arc::new(rb);


    rocket::build()
        .mount("/api", user_urls::routes())
        .mount("/api", iorecord_urls::routes())
        .mount("/api", dis_urls::routes())
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await;
}