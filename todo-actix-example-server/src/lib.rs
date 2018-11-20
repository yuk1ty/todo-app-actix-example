#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod schema;
