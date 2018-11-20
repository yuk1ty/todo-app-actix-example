#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
