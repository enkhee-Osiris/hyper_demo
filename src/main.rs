mod helpers;
mod models;
mod routes;
mod services;

use models::todo::Db;
use routes::todos;

use std::sync::Arc;
use tokio::sync::Mutex;

fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

#[tokio::main]
async fn main() {
    let db = blank_db();

    let api = todos(db);
    let routes = api;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
