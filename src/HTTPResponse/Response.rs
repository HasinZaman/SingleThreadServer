use crate::HTTPRequest::HTTPBody::Body;
use super::ResponseStatusCode::*;

pub struct Response {
    status : ResponseStatusCode,
    body : Option<Body>
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut tmp : String = format!("{}\r", self.status.to_string());

        tmp
    }
}