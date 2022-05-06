#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use std::string::String;
use std::sync::Arc;

use rbatis::crud::CRUD;
use rbatis::Json;
use rbatis::rbatis::Rbatis;
use rocket::{Request, State};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{content, status};
use serde;
use serde_json;

use crate::user::models::User;
use crate::user::urls;

mod user;

#[rocket::main]
async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    log::info!("linking database...");
    let rb = Rbatis::new();

    rb.link("sqlite:///home/satan/library/db.sqlite3").await.unwrap();
    let rb = Arc::new(rb);


    rocket::build()
        .mount("/api", routes![urls::login])
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await;
}