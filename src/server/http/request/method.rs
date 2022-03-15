use super::super::body::Body;
use super::parser_error::ParserError;

#[derive(Debug)]
pub enum Method {
    Get { file: String },
    Head { file: String },
    Post { file: String, body: Body },
    Put { file: String, body: Body },
    Delete { file: String, body: Option<Body> },
    Connect { url: String },
    Options { url: String },
    Trace { file: String },
    Patch { file: String, body: Body },
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

        match method.to_ascii_uppercase().as_str() {
            "GET" => {
                return Result::Ok(Method::Get {
                    file: target.to_string(),
                })
            }
            "HEAD" => {
                return Result::Ok(Method::Head {
                    file: target.to_string(),
                })
            }
            "POST" => {
                return Result::Ok(Method::Post {
                    file: target.to_string(),
                    body: body_unwrap(body)?,
                })
            }
            "PUT" => {
                return Result::Ok(Method::Put {
                    file: target.to_string(),
                    body: body_unwrap(body)?,
                })
            }
            "DELETE" => {
                return Result::Ok(Method::Delete {
                    file: target.to_string(),
                    body: body,
                })
            }
            "CONNECT" => {
                return Result::Ok(Method::Connect {
                    url: target.to_string(),
                })
            }
            "OPTIONS" => {
                return Result::Ok(Method::Options {
                    url: target.to_string(),
                })
            }
            "TRACE" => {
                return Result::Ok(Method::Trace {
                    file: target.to_string(),
                })
            }
            "PATCH" => {
                return Result::Ok(Method::Patch {
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
        #[allow(unused_variables)]
        match &self {
            Method::Get { file } => {
                return String::from(format!("GET file={}", file));
            }
            Method::Head { file } => {
                return String::from(format!("Head file={}", file));
            }
            Method::Post { file, body } => {
                return String::from(format!("POST file={}", file));
            }
            Method::Put { file, body } => {
                return String::from(format!("PUT file={}", file));
            }
            Method::Delete { file, body } => {
                return String::from(format!("DELETE file={}", file));
            }
            Method::Connect { url } => {
                return String::from(format!("CONNECT URL={}", url));
            }
            Method::Options { url } => {
                return String::from(format!("OPTIONS URL={}", url));
            }
            Method::Trace { file } => {
                return String::from(format!("TRACE file={}", file));
            }
            Method::Patch { file, body } => {
                return String::from(format!("PATCH file={}", file));
            }
        }
    }
}
