use std::env;
use std::thread;
use std::time::Duration;
use docker_rust_seed::ThreadPool;
use serde_json;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

#[macro_use] extern crate serde_derive;

enum StatusCode {
    Ok = 200,
    NotFound = 404
}

#[derive(Serialize)]
struct Response {
    success: bool,
    message: String
}

fn main() {
    let addr = String::from("0.0.0.0");
    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let listener = TcpListener::bind(addr + ":" + &port).unwrap();

    let pool = ThreadPool::new(10);

    print!("Server Started \n");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
       
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_code, response_data) =
        if buffer.starts_with(get) {
            (
                StatusCode::Ok,
                Response {
                    success: true,
                    message: "Hello from Rust".to_string()
                }
            )
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            (
                StatusCode::Ok,
                Response {
                    success: true,
                    message: "Sleep from Rust".to_string()
                }
            )
        } else {
            (
                StatusCode::NotFound,
                Response {
                    success: false,
                    message: "Error from Rust".to_string()
                }
            )
        };

    let status_line = match status_code {
        StatusCode::Ok => "HTTP/1.1 200 OK",
        StatusCode::NotFound => "HTTP/1.1 404 NOT FOUND",
    };

    let contents = serde_json::to_string(&response_data).expect("Serialization failed");
    let content_type = "Content-type: application/json";
    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, content_type, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
