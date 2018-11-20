use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use api::context::AppState;
use domain::repository::todo::FindAllTasks;
use futures::Future;

/// このアプリケーションがデータベースに保有しているすべての Todo タスクを取得し，それをレスポンスとして返します．
pub fn find_all_todos(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(FindAllTasks)
        .from_err()
        .and_then(move |res| match res {
            Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
            Err(error) => Err(error),
        }).responder()
}
