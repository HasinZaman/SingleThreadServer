use crate::HTTPRequest::Method::Method;
use crate::HTTPResponse::{Response::Response, ResponseStatusCode::ResponseStatusCode};

use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;

pub struct MethodLogic {
    pub get : Box<dyn Fn(Method)->Response>,
    pub head : Box<dyn Fn(Method)->Response>,
    pub post : Box<dyn Fn(Method)->Response>,
    pub put : Box<dyn Fn(Method)->Response>,
    pub delete : Box<dyn Fn(Method)->Response>,
    pub connect : Box<dyn Fn(Method)->Response>,
    pub option : Box<dyn Fn(Method)->Response>,
    pub trace : Box<dyn Fn(Method)->Response>,
    pub patch : Box<dyn Fn(Method)->Response>,
}

impl MethodLogic {
    pub fn default_logic() -> Box<dyn Fn(Method)->Response> {
        Box::new(
            |_request : Method | {
                Response {
                    status : ResponseStatusCode::MethodNotAllowed,
                    body : Option::None
                }
            }
        )
    }
}

pub fn start(method_action : MethodLogic) {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &method_action);
    }
}

fn handle_connection(mut stream: TcpStream, method_action : &MethodLogic) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    
    let method = Method::new(buffer);
    
    let response : Response;

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
