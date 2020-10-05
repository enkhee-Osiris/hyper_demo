extern crate pretty_env_logger;
extern crate warp;

#[macro_use]
extern crate log;

use std::convert::Infallible;
use std::error::Error;

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

// extern crate log;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Osiris {
    pub name: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    code: u16,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

impl std::fmt::Display for Osiris {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value name: {})", self.name)
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // curl http://localhost:3030/hi
    let hi = warp::path("hi").and(warp::get()).map(|| "Hello, World!");

    // curl -X POST -H "Accept: application/json" -H "Content-Type: application/json" -d '{"name": "osiris", "rate": 3}' http://localhost:3030/hello/2
    let hello = warp::path!("hello" / u32)
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    // curl -X POST -H "Accept: application/json" -H "Content-Type: application/json" -d '{"name": 3}' http://localhost:3030/osiris
    let osiris_test = warp::path!("osiris")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: Osiris| {
            warp::reply::json(&ResponseMessage {
                code: 0,
                message: "SUCCESSFUL".into(),
            })
        });

    warp::serve(hi.or(hello).or(osiris_test.recover(handle_rejection)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("name") {
                    "FIELD_ERROR: name"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ResponseMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
