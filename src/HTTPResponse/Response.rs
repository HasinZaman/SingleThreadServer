use crate::HTTPRequest::HTTPBody::Body;
use super::ResponseStatusCode::*;

pub struct Response {
    pub status : ResponseStatusCode,
    pub body : Option<Body>
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut tmp : String = format!("{}", self.status.to_string());

        match &self.body {
            Some(body) => {
                tmp.push_str("\n");

                let content_length = format!("Content-Length: {}\n", body.content.len());
                tmp.push_str(&content_length);

                let content_type = format!("Content-Type: {}\n", body.content_type.to_string());
                tmp.push_str(&content_type);

                let content = format!("\n{}", &(body.content));
                tmp.push_str(&content);
            },
            None => {},
        }

        tmp
    }
}