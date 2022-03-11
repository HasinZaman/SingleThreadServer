use super::Body::{Body, ContentType};
use super::Request::ParserError::ParserError;

#[derive(Debug)]
pub enum Method {
    GET {
        file : String,
    },
    HEAD {
        file : String,
    },
    POST {
        file : String,
        body : Body,
    },
    PUT {
        file : String,
        body : Body,
    },
    DELETE {
        file : String,
        body : Option<Body>,
    },
    CONNECT {
        URL : String,
    },
    OPTIONS {
        URL : String,
    },
    TRACE {
        file : String,
    },
    PATCH {
        file : String,
        body : Body,
    },
}

impl Method {
    pub fn new(request_data : [u8; 1024]) -> Result<Method, ParserError> {
        let body_unwrap = |body_tmp : Option<Body>| -> Result<Body, ParserError> {
            match body_tmp {
                Some(body) => return Result::Ok(body),
                None => return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("Http request requires body attribute"))))
            }
        };

        let request : String = String::from_utf8_lossy(&request_data[..]).to_string();
        let request : Vec<&str> = request.split("\n").collect();

        if request.len() == 0 {
            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("request must have more than 0 lines"))));
        }

        let method;
        let target;
        let version;
        
        match Method::get_start_line(&request[0]) {
            Ok(tmp) => {
                method = tmp.0;
                target = tmp.1;
                version = tmp.2;
            },
            Err(tmp) => return Result::Err(tmp),
        }

        let method : String = method.to_uppercase();

        match method.as_str() {
            "GET"       => return Result::Ok(Method::GET{ file : target.to_string() }),
            "HEAD"      => return Result::Ok(Method::HEAD{ file : target.to_string() }),
            "POST"      => return Result::Ok(Method::POST{ file : target.to_string(), body : body_unwrap(Method::get_body(&request)?)?}),
            "PUT"       => return Result::Ok(Method::PUT{ file : target.to_string(), body : body_unwrap(Method::get_body(&request)?)?}),
            "DELETE"    => return Result::Ok(Method::DELETE{ file : target.to_string(), body : Method::get_body(&request)?}),
            "CONNECT"   => return Result::Ok(Method::CONNECT{ URL : target.to_string() }),
            "OPTIONS"   => return Result::Ok(Method::OPTIONS{ URL : target.to_string() }),
            "TRACE"     => return Result::Ok(Method::TRACE{ file : target.to_string() }),
            "PATCH"     => return Result::Ok(Method::PATCH{ file : target.to_string(), body : body_unwrap(Method::get_body(&request)?)?}),
            _           => return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(format!("{} is an invalid method name", method.as_str()))))),
        }
    }

    fn get_start_line<'a>(start_line: &'a str) -> Result<(&'a str, &'a str, &'a str), ParserError> {
        let mut start_line = start_line.split_whitespace();
        
        let method : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;
        let target : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;
        let version : &str = start_line.next().ok_or(ParserError::InvalidMethod(Option::None))?;

        Result::Ok((method, target, version))
    }

    fn get_body<'a>(request : &Vec<&str>) -> Result<Option<Body>, ParserError> {
        let mut content_type : Option<ContentType> = Option::None;
        let mut boundary_index : Option<usize> = Option::None;

        //get content type
        for i1 in 1..request.len() {
            match request[i1].split_once(":") {
                Some((key, value)) => {
                    if key.to_lowercase() == "content-type" {
                        let content_type_result = ContentType::new(value.trim_start());
        
                        match content_type_result {
                            Ok( tmp ) => {
                                content_type = Option::Some(tmp);
                            },
                            Err(err) => return Result::Err(err),
                        }
                    }
                },
                None => {
                    if &request[i1] == &"" {
                        if content_type.is_none() {
                            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from("No content type before beginning of body"))))
                        }
        
                        boundary_index = Option::Some(i1);
                        break;
                    }
                }
            }
        };

        if content_type.is_none() {
            return Result::Ok(Option::None) 
        }

        let mut body : String = String::from("");

        match boundary_index {
            Some(tmp) => {
                for i1 in tmp..request.len() {
                    println!("{} : {}", i1, request[i1]);
                    if request[i1].contains("\u{0}") {
                        body.push_str(request[i1].trim_end_matches('\u{0}'));
                        break;
                    }
                    body.push_str(request[i1]);
                }
            },
            None => panic!("boundary_index is invalid type (None)"),
        }
        match content_type {
            Some( tmp ) => return Result::Ok(Option::Some(Body { content_type : tmp, content : body })),
            None => panic!("content_type is invalid type (None)"),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match &self {
            Method::GET { file } => {
                return String::from(format!("GET file={}", file));
            },
            Method::HEAD { file } => {
                return String::from(format!("Head file={}", file));
            },
            Method::POST { file, body } => {
                return String::from(format!("POST file={}", file));
            },
            Method::PUT { file, body } => {
                return String::from(format!("PUT file={}", file));
            },
            Method::DELETE { file, body } => {
                return String::from(format!("DELETE file={}", file));
            },
            Method::CONNECT { URL } => {
                return String::from(format!("CONNECT URL={}", URL));
            },
            Method::OPTIONS { URL } => {
                return String::from(format!("OPTIONS URL={}", URL));
            },
            Method::TRACE { file } => {
                return String::from(format!("TRACE file={}", file));
            },
            Method::PATCH { file, body } => {
                return String::from(format!("PATCH file={}", file));
            }
        }
    }
}