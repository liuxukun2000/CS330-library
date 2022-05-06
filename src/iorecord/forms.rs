use chrono::{Date, NaiveDate, TimeZone};
use rocket::form::{Errors, Form};
use rocket::form;
use rocket::form::{Error, FromForm};
use rocket::serde::{Deserialize, Serialize};

// use rocket::tokio::time::error::Error;

#[derive(FromForm, Deserialize, Serialize)]
pub struct IOQueryForm {
    #[field(validate = check_cell(& self.end, & self.cell, & self.library))]
    pub start: String,
    pub end: String,
    pub cell: String,
    pub library: String,
}

fn check_cell<'v>(start: &str, end: &str, cell: &str, library: &str) -> form::Result<'v, ()> {
    log::info!("in in in");
    let mut fmt = "";
    match cell {
        "day" => fmt = "%Y-%m-%d",
        "month" => fmt = "%Y-%m",
        _ => ()
    }
    if fmt.len() == 0 {
        return Err(Errors::new());
    }
    let start = NaiveDate::parse_from_str(start, &fmt);
    let end = NaiveDate::parse_from_str(end, &fmt);
    if start.is_err() || end.is_err() {
        return Err(Error::validation("Invalid Time"))?;
    }
    if start.unwrap() >= end.unwrap() {
        return Err(Error::validation("Invalid Time"))?;
    }
    Ok(())
}