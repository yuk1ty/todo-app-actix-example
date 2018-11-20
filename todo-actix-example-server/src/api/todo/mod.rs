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
use domain::repository::todo::FindAllTasks;
use futures::future;
use futures::Future;
use infrastructure::mysql::MySqlCli;
use std::sync::Arc;

/// このアプリケーションがデータベースに保有しているすべての Todo タスクを取得し，それをレスポンスとして返します．
pub fn find_all_todos(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(FindAllTasks)
        .from_err()
        .and_then(move |res| match res {
            Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
            Err(err) => {
                error!("{}", err.to_string());
                Err(err)
            },
        }).responder()
}

pub fn preflight_insert_todo(_: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    future::ok(HttpResponse::Ok().finish()).responder()
}

pub fn insert_todo(
    (target, req): (Json<NewTodo>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let addr: Arc<Addr<MySqlCli>> = req.state().db.clone();
    let actor = addr.send(CreateNewTask {
        title: target.0.title,
    });

    actor
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("{:?}", err)))
        .and_then(|r| match r {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => {
                error!("{}", err.to_string());
                Ok(HttpResponse::InternalServerError().finish())
            }
        }).responder()
}
