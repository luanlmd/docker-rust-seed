use std::fs;
use std::env;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

enum StatusCode {
    Ok = 200,
    NotFound = 404
}
fn main() {
    let addr = String::from("0.0.0.0");
    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let listener = TcpListener::bind(addr + ":" + &port).unwrap();

    print!("Server Started \n");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_code, filename) =
        if buffer.starts_with(get) {
            (StatusCode::Ok, "index.html")
        } else {
            (StatusCode::NotFound, "404.html")
        };

    let status_line = match status_code {
        StatusCode::Ok => "HTTP/1.1 200 OK",
        StatusCode::NotFound => "HTTP/1.1 404 NOT FOUND",
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
