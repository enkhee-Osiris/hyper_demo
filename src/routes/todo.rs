use crate::helpers::{json_body, with_db};
use crate::models::todo::Db;
use crate::services::todo::{create_todo, delete_todo, list_todos, update_todo};

use warp::{Filter, Rejection, Reply};

/// GET /todos
pub fn todos_list(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(with_db(db))
        .and_then(list_todos)
}

/// POST /todos with JSON body
pub fn todos_create(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(create_todo)
}

/// PUT /todos/:id with JSON body
pub fn todos_update(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(update_todo)
}

/// DELETE /todos/:id
pub fn todos_delete(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("todos" / u64)
        .and(warp::delete())
        .and(admin_only)
        .and(with_db(db))
        .and_then(delete_todo)
}
