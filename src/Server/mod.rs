use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;

use chrono;

pub mod FileReader;
pub mod HTTP;
pub mod MethodLogic;

use HTTP::Method::Method;
use HTTP::Response::Response::Response;
use HTTP::Response::ResponseStatusCode::ResponseStatusCode;

use std::collections::HashMap;

pub fn start(method_action: MethodLogic::MethodLogic) {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &method_action, "localhost");
    }
}

#[allow(unused_variables, non_snake_case, unreachable_patterns)]
fn handle_connection(
    mut stream: TcpStream,
    method_action: &MethodLogic::MethodLogic,
    domain: &str,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let method: Method;
    let meta_data: HashMap<String, String>;
    let response: Response;

    match HTTP::Request::parse(buffer, &domain) {
        Ok(val) => {
            println!("Success");
            method = val.0;
            meta_data = val.1;
            response = handle_method(method, method_action);
        }
        Err(err) => {
            println!("Failure:{:?}", err);
            response = Response {
                status: ResponseStatusCode::BadRequest,
                body: Option::None,
            };
        }
    }
    println!("{:?}", chrono::offset::Utc::now());

    println!("response:\n{}\n", response.to_string());

    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[allow(unused_variables)]
fn handle_method(method: Method, method_action: &MethodLogic::MethodLogic) -> Response {
    match &method {
        GET => (method_action.get)(method),
        HEAD => (method_action.head)(method),
        POST => (method_action.post)(method),
        PUT => (method_action.put)(method),
        DELETE => (method_action.delete)(method),
        CONNECT => (method_action.connect)(method),
        OPTIONS => (method_action.option)(method),
        TRACE => (method_action.trace)(method),
        PATCH => (method_action.patch)(method),
        _ => Response {
            status: ResponseStatusCode::InternalServerError,
            body: Option::None,
        },
    }
}
