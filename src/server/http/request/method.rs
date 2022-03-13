use super::super::body::{Body, ContentType};
use super::parser_error::ParserError;

#[derive(Debug)]
pub enum Method {
    GET { file: String },
    HEAD { file: String },
    POST { file: String, body: Body },
    PUT { file: String, body: Body },
    DELETE { file: String, body: Option<Body> },
    CONNECT { URL: String },
    OPTIONS { URL: String },
    TRACE { file: String },
    PATCH { file: String, body: Body },
}

impl Method {
    pub fn new(method: String, target: String, body: Option<Body>) -> Result<Method, ParserError> {
        let body_unwrap = |body_tmp: Option<Body>| -> Result<Body, ParserError> {
            match body_tmp {
                Some(body) => return Result::Ok(body),
                None => {
                    return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(
                        "Http request requires body attribute",
                    ))))
                }
            }
        };

        match method.as_str() {
            "GET" => {
                return Result::Ok(Method::GET {
                    file: target.to_string(),
                })
            }
            "HEAD" => {
                return Result::Ok(Method::HEAD {
                    file: target.to_string(),
                })
            }
            "POST" => {
                return Result::Ok(Method::POST {
                    file: target.to_string(),
                    body: body_unwrap(body)?,
                })
            }
            "PUT" => {
                return Result::Ok(Method::PUT {
                    file: target.to_string(),
                    body: body_unwrap(body)?,
                })
            }
            "DELETE" => {
                return Result::Ok(Method::DELETE {
                    file: target.to_string(),
                    body: body,
                })
            }
            "CONNECT" => {
                return Result::Ok(Method::CONNECT {
                    URL: target.to_string(),
                })
            }
            "OPTIONS" => {
                return Result::Ok(Method::OPTIONS {
                    URL: target.to_string(),
                })
            }
            "TRACE" => {
                return Result::Ok(Method::TRACE {
                    file: target.to_string(),
                })
            }
            "PATCH" => {
                return Result::Ok(Method::PATCH {
                    file: target.to_string(),
                    body: body_unwrap(body)?,
                })
            }
            _ => {
                return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(
                    format!("{} is an invalid method name", method.as_str()),
                ))))
            }
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match &self {
            Method::GET { file } => {
                return String::from(format!("GET file={}", file));
            }
            Method::HEAD { file } => {
                return String::from(format!("Head file={}", file));
            }
            Method::POST { file, body } => {
                return String::from(format!("POST file={}", file));
            }
            Method::PUT { file, body } => {
                return String::from(format!("PUT file={}", file));
            }
            Method::DELETE { file, body } => {
                return String::from(format!("DELETE file={}", file));
            }
            Method::CONNECT { URL } => {
                return String::from(format!("CONNECT URL={}", URL));
            }
            Method::OPTIONS { URL } => {
                return String::from(format!("OPTIONS URL={}", URL));
            }
            Method::TRACE { file } => {
                return String::from(format!("TRACE file={}", file));
            }
            Method::PATCH { file, body } => {
                return String::from(format!("PATCH file={}", file));
            }
        }
    }
}
