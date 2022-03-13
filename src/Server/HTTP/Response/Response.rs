use super::ResponseStatusCode::*;
use crate::Server::HTTP::Body::Body;

pub struct Response {
    pub status: ResponseStatusCode,
    pub body: Option<Body>,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut tmp: String = format!("HTTP/1.1 {}", self.status.to_string());

        match &self.body {
            Some(body) => {
                tmp.push_str("\r\n");

                let content_length = format!("Content-Length: {}\r\n", body.content.len());
                tmp.push_str(&content_length);

                let content_type = format!("Content-Type: {}\r\n", body.content_type.to_string());
                tmp.push_str(&content_type);

                let content = format!("\n{}", &(body.content));
                tmp.push_str(&content);
            }
            None => {}
        }

        tmp
    }
}
