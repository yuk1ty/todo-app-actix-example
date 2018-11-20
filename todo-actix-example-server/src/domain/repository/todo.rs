use actix::Handler;
use actix::Message;
use actix_web::error::ErrorInternalServerError;
use actix_web::Error as AWError;
use diesel;
use diesel::prelude::*;
use domain::model::todo::NewTodo;
use domain::model::todo::Todo;
use infrastructure::mysql::MySqlCli;
use schema::todos::dsl::*;
use schema::{todos, todos::dsl::todos as all_todos};
use std::ops::Deref;

/// データベースに保有しているすべての Todo リストを返します．
pub struct FindAllTasks;

impl Message for FindAllTasks {
    type Result = Result<Vec<Todo>, AWError>;
}

impl Handler<FindAllTasks> for MySqlCli {
    type Result = Result<Vec<Todo>, AWError>;

    fn handle(&mut self, _: FindAllTasks, _: &mut Self::Context) -> Self::Result {
        all_todos
            .load::<Todo>(self.get_conn()?.deref())
            .map_err(|_| ErrorInternalServerError("Failed to get task"))
    }
}

pub struct CreateNewTask {
    pub title: String,
}

impl Message for CreateNewTask {
    type Result = Result<(), AWError>;
}

impl Handler<CreateNewTask> for MySqlCli {
    type Result = Result<(), AWError>;

    fn handle(&mut self, todo: CreateNewTask, _: &mut Self::Context) -> Self::Result {
        let new_todo = NewTodo {
            title: todo.title,
            status: false,
        };

        // https://github.com/diesel-rs/diesel/issues/734
        // これにより，登録後のタスクを単体で取り出すことができないらしい…
        diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(self.get_conn()?.deref())
            .map(|_| ())
            .map_err(|err| ErrorInternalServerError(err))
    }
}
