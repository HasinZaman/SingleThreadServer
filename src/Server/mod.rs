use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;

use chrono;

pub mod MethodLogic;
pub mod FileReader;
pub mod HTTP;

use HTTP::Method::Method as Method;
use HTTP::Response::Response::Response as Response;
use HTTP::Response::ResponseStatusCode::ResponseStatusCode as ResponseStatusCode;

pub fn start(method_action : MethodLogic::MethodLogic) {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &method_action);
    }
}


#[allow(unused_variables, non_snake_case, unreachable_patterns)]
fn handle_connection(mut stream: TcpStream, method_action : &MethodLogic::MethodLogic) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let method = Method::new(buffer);
    
    let response : Response;
    
    println!("{:?}", chrono::offset::Utc::now());
    match method {
        Ok(request) => {
            println!("request:\n{:?}", &request);
            response = match &request {
                GET => (method_action.get)(request),
                HEAD => (method_action.head)(request),
                POST => (method_action.post)(request),
                PUT => (method_action.put)(request),
                DELETE => (method_action.delete)(request),
                CONNECT => (method_action.connect)(request),
                OPTIONS => (method_action.option)(request),
                TRACE => (method_action.trace)(request),
                PATCH => (method_action.patch)(request),
                _ => Response{ status : ResponseStatusCode::InternalServerError, body : Option::None },
            }
        },
        Err(_error) => {
            println!("{:?}", _error);
            response = Response{
                status : ResponseStatusCode::BadRequest,
                body : Option::None
            };
        }
    }

    println!("response:\n{}\n", response.to_string());
    
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
