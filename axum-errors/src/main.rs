use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(get_root))
        .route("/count", get(get_count))
        .fallback(fallback);

    println!("Starting server listening on: 'http://127.0.0.1:3000'");
    axum::Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_root() -> Result<String, MyError> {
    if random() {
        return Err(MyError::DatabaseError);
    } else {
        return Err(MyError::CountingError);
    }
}

async fn get_count() -> Result<String, CountingError> {
    Err(CountingError::new(1234))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Page not found: {}", uri))
}

#[derive(Deserialize, Serialize)]
struct CountingError {
    errno: u64,
    message: String,
    value: u64,
}

impl CountingError {
    fn new(value: u64) -> Self {
        Self {
            errno: 4001,
            message: "There was a counting error.".to_string(),
            value,
        }
    }
}

impl IntoResponse for CountingError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

#[derive(Clone, Copy)]
enum MyError {
    DatabaseError = 1001,
    CountingError = 4001,
}

impl MyError {
    fn errno(&self) -> u64 {
        *self as u64
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            MyError::DatabaseError => {
                ErrorBody::new(self.errno(), "There was a database error".to_string())
            }
            MyError::CountingError => {
                ErrorBody::new(self.errno(), "There was a counting error".to_string())
            }
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

#[derive(Deserialize, Serialize)]
struct ErrorBody {
    errno: u64,
    message: String,
}

impl ErrorBody {
    fn new(errno: u64, message: String) -> Self {
        Self { errno, message }
    }
}
