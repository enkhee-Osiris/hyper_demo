use crate::models::todo::{Db, Todo};

use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Reply;

pub async fn list_todos(db: Db) -> Result<impl Reply, Infallible> {
    let todos = db.lock().await;
    let todos: Vec<Todo> = todos.clone();

    Ok(warp::reply::json(&todos))
}

pub async fn create_todo(create: Todo, db: Db) -> Result<impl Reply, Infallible> {
    let mut vec = db.lock().await;

    for todo in vec.iter() {
        if todo.id == create.id {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    vec.push(create);

    Ok(StatusCode::CREATED)
}

pub async fn update_todo(id: u64, update: Todo, db: Db) -> Result<impl Reply, Infallible> {
    let mut vec = db.lock().await;

    for todo in vec.iter_mut() {
        if todo.id == id {
            *todo = update;
            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_todo(id: u64, db: Db) -> Result<impl Reply, Infallible> {
    let mut vec = db.lock().await;

    let len = vec.len();
    vec.retain(|todo| todo.id != id);

    let deleted = vec.len() != len;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
