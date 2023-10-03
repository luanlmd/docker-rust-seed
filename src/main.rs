use std::thread;
use std::time::Duration;
use serde_derive::Serialize;
use std::net::SocketAddr;
use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};

#[derive(Serialize)]
struct Response {
    success: bool,
    message: String
}

#[tokio::main]

async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/sleep", get(sleep));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> (StatusCode, Json<Response>)
{
    let response = Response {
        success: true,
        message: "Hello from Rust".to_string()
    };

    (StatusCode::ACCEPTED, Json(response))
}

async fn sleep() -> (StatusCode, Json<Response>) {
    thread::sleep(Duration::from_secs(5));
    let response = Response {
        success: true,
        message: "Sleep from Rust".to_string()
    };

    (StatusCode::OK, Json(response))
}
