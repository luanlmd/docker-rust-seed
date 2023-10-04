use std::env;
use std::time::Duration;
use serde_derive::Serialize;
use std::net::SocketAddr;
use axum::{ routing::get, middleware::{self, Next}, http::{StatusCode, Request}, Json, Router, response::IntoResponse };

#[derive(Serialize)]
struct Response {
    success: bool,
    message: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/sleep", get(sleep))
    .layer(middleware::from_fn(log_requets));

    let port = env::var("PORT").unwrap_or(String::from("3000")).parse::<u16>().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Starting server on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn log_requets(req: Request<axum::body::Body>, next: Next<axum::body::Body>) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
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
    tokio::time::sleep(Duration::from_secs(5)).await;

    let response = Response {
        success: true,
        message: "Sleep from Rust".to_string()
    };

    (StatusCode::OK, Json(response))
}
