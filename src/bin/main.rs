extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let hello_api = b"GET /hello HTTP/1.1\r\n";

    let (status, content) = if buffer.starts_with(hello_api) {
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };

    let response = format!("{}{}", status, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
