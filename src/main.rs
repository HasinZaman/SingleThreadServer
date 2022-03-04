use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use HTTPRequest::Method::Method;

//import HTTPRequest
mod HTTPRequest;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    println!("--Request--\n{}", String::from_utf8_lossy(&buffer[..]));
    
    match Method::new(buffer) {
        Ok(request) => {
            println!("{:#?}", request);
        },
        Err(error) => {
            println!("{:#?}", error);
        }
    }
    //println!("--Parse Result--\n{}",requestParser(&stream))
}
