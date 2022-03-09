use crate::HTTPRequest::{Method::Method, HTTPBody};
use crate::HTTPResponse::{Response::Response, ResponseStatusCode::ResponseStatusCode};
use crate::FileReader;

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
    pub fn default_not_allowed_logic() -> Box<dyn Fn(Method)->Response> {
        Box::new(
            |_request : Method | {
                Response {
                    status : ResponseStatusCode::MethodNotAllowed,
                    body : Option::None
                }
            }
        )
    }

    pub fn default_get_logic() -> Box<dyn Fn(Method)->Response> {
        Box::new(
            |request : Method | {
                match request {
                    Method::GET{ file } => {
                        let path_buf = FileReader::parse(&file);

                        let file_name = match &path_buf.file_name(){
                            Some(name) => name.to_str().unwrap(),
                            None => panic!("FileReader::parse does not return file name. {:?}", path_buf),
                        };

                        let body = FileReader::get_file_content(&path_buf).unwrap();

                        match file_name {
                            "404.html" => {
                                return Response {
                                    status : ResponseStatusCode::NotFound,
                                    body : Option::Some(
                                        HTTPBody::Body {
                                            content_type: HTTPBody::ContentType::Text{
                                                value: HTTPBody::value::Text::html
                                            },
                                            content: body,
                                        }
                                    )
                                } 
                            },
                            _ => {
                                let content_type = path_buf.extension().unwrap();
                                let content_type = content_type.to_str().unwrap();

                                let content_type : HTTPBody::ContentType = match content_type {
                                    "gif" => {
                                        HTTPBody::ContentType::Image{
                                            value: HTTPBody::value::Image::gif
                                        }
                                    },
                                    "jpg" | "jpeg" | "jpe" | "jfif" => {
                                        HTTPBody::ContentType::Image{
                                            value: HTTPBody::value::Image::jpeg
                                        }
                                    },
                                    "png" => {
                                        HTTPBody::ContentType::Image{
                                            value: HTTPBody::value::Image::png
                                        }
                                    },
                                    "tif" | "tiff" => {
                                        HTTPBody::ContentType::Image{
                                            value: HTTPBody::value::Image::tiff
                                        }
                                    },
                                    "css" => {
                                        HTTPBody::ContentType::Text{
                                            value: HTTPBody::value::Text::css
                                        }
                                    },
                                    "csv" => {
                                        HTTPBody::ContentType::Text{
                                            value: HTTPBody::value::Text::csv
                                        }
                                    },
                                    "html" => {
                                        HTTPBody::ContentType::Text{
                                            value: HTTPBody::value::Text::html
                                        }
                                    },
                                    "javascript" => {
                                        HTTPBody::ContentType::Text{
                                            value: HTTPBody::value::Text::javascript
                                        }
                                    },
                                    "xml" => {
                                        HTTPBody::ContentType::Text{
                                            value: HTTPBody::value::Text::xml
                                        }
                                    },
                                    "mpeg" => {
                                        HTTPBody::ContentType::Video{
                                            value: HTTPBody::value::Video::mpeg
                                        }
                                    },
                                    "mp4" => {
                                        HTTPBody::ContentType::Video{
                                            value: HTTPBody::value::Video::mp4
                                        }
                                    },
                                    "webm" => {
                                        HTTPBody::ContentType::Video{
                                            value: HTTPBody::value::Video::webm
                                        }
                                    },
                                    _=>{
                                        return Response {
                                            status : ResponseStatusCode::NotFound,
                                            body : Option::None
                                        } 
                                    }
                                };



                                return Response {
                                    status : ResponseStatusCode::Ok,
                                    body : Option::Some(
                                        HTTPBody::Body {
                                            content_type: content_type,
                                            content: body,
                                        }
                                    )
                                } 
                            }
                        }
                    },
                    _=> {
                        Response {
                            status : ResponseStatusCode::BadRequest,
                            body : Option::None
                        } 
                    }
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
