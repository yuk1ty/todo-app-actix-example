extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate todo_actix_example_server;

use actix::SyncArbiter;
use actix::System;
use actix_web::http::Method;
use actix_web::server;
use actix_web::App;
use dotenv::dotenv;
use std::env;
use todo_actix_example_server::api;
use todo_actix_example_server::api::all_todos;
use todo_actix_example_server::api::insert_todo;
use todo_actix_example_server::infrastructure::mysql;
use todo_actix_example_server::infrastructure::mysql::MySqlCli;

const NUM_DB_THREADS: usize = 3;

fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();

    let sys = System::new("todo_app_example");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = mysql::init_pool(&database_url).expect("failed to create connection pool");
    let addr = SyncArbiter::start(NUM_DB_THREADS, move || MySqlCli::new(pool.clone()));

    let app = move || {
        App::with_state(api::context::AppState::new(addr.clone()))
            .resource("/hc", |r| r.get().with_async(api::hc))
            .resource("/todos", |r| r.get().with_async(all_todos::find_all_todos))
            .resource("/todos/insert", |r| {
                r.post().with_async(insert_todo::insert_todo)
            }).resource("/todos/insert", |r| {
                r.method(Method::OPTIONS)
                    .with_async(insert_todo::preflight_insert_todo)
            })
    };

    server::new(app).bind("127.0.0.1:8088").unwrap().start();

    sys.run();
}
