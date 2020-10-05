use self::todo::{todos_create, todos_delete, todos_list, todos_update};
use crate::models::todo::Db;

use warp::{Filter, Rejection, Reply};

pub mod todo;

pub fn todos(db: Db) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    todos_list(db.clone())
        .or(todos_create(db.clone()))
        .or(todos_update(db.clone()))
        .or(todos_delete(db))
}
