use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let hello_api = b"GET /hello HTTP/1.1\r\n";

    let (status,content) = if buffer.starts_with(hello_api){
        ("HTTP/1.1 200 OK\r\n\r\n","Hello")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };

    let response = format!("{}{}",status, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}