use actix::Addr;
use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;
use api::context::AppState;
use domain::model::todo::NewTodo;
use domain::repository::todo::CreateNewTask;
use futures::future;
use futures::Future;
use infrastructure::mysql::MySqlCli;
use std::sync::Arc;

pub fn preflight_insert_todo(_: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    future::ok(HttpResponse::Ok().finish()).responder()
}

pub fn insert_todo(
    (target, req): (Json<NewTodo>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let addr: Arc<Addr<MySqlCli>> = req.state().db.clone();
    let actor = addr.send(CreateNewTask {
        title: target.0.title,
        body: target.0.body,
    });

    actor
        .map_err(|_| actix_web::error::ErrorInternalServerError("err"))
        .and_then(|r| match r {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => {
                println!("{}", err.to_string());
                Ok(HttpResponse::InternalServerError().finish())
            }
        }).responder()
}
