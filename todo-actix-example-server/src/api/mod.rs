use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use api::context::AppState;
use futures::future;

pub mod all_todos;
pub mod context;
pub mod insert_todo;

pub fn hc(_: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    future::ok(HttpResponse::Ok().body("OK")).responder()
}
