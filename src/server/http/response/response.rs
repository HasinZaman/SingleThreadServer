//! response module is responsible for structs and functions related to HTTP responses
use super::response_status_code::*;
use crate::server::http::body::Body;
use std::collections::HashMap;

/// Response struct define the Response structure
pub struct Response {
    pub status: ResponseStatusCode,
    pub meta_data : HashMap<String, String>,
    pub body: Option<Body>,
}

macro_rules! append_to {
    ($list:ident, $value:expr) => {
        for byte in $value.as_bytes() {
            $list.push(byte.clone());
        }
    };
    ($list:ident, $value:ident) => {
        for byte in $value {
            $list.append(byte.clone());
        }
    };
}

impl Response {
    /// as_bytes provides a bytes required in order send response
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        append_to!(output, format!("HTTP/1.1 {}\r\n", self.status.to_string()));

        self.meta_data
            .keys()
            .into_iter().
            map(
                |key| -> Option<u8> {
                    append_to!(
                        output,
                        format!("{}: {}\r\n", key, &self.meta_data.get(key).unwrap())
                    );
                    None
            })
            .last();

        match &self.body {
            Some(body) => {

                append_to!(
                    output,
                    format!("Content-Length: {}\r\n", body.content.len())
                );

                append_to!(
                    output,
                    format!("Content-Type: {}\r\n", body.content_type.to_string())
                );

                append_to!(output, "\n");

                output.append(&mut body.content.clone());
            }
            None => {}
        }

        output
    }
}
