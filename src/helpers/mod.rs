use crate::models::todo::{Db, Todo};

use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body() -> impl Filter<Extract = (Todo,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
