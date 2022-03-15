use super::super::body::{Body, ContentType};
use super::method::Method;
use super::parser_error::ParserError;

use std::collections::HashMap;
use std::str::Split;

pub fn parse(
    request_data: [u8; 1024],
    domain: &str,
) -> Result<(Method, HashMap<String, String>), ParserError> {
    let request: String = String::from_utf8_lossy(&request_data[..]).to_string();

    println!("{:?}", request);

    let mut request = request.split("\n");

    let (method, target, version) = match get_start_line(request.next()) {
        Ok(ok) => ok,
        Err(err) => return Result::Err(err),
    };

    let method = method.to_uppercase();

    let (body, meta_data) = get_data(request, domain)?;

    let method: Method = Method::new(method.to_string(), target.to_string(), body)?;

    println!("\n\n{:?}\n{:?}\n\n", method, meta_data);
    Result::Ok((method, meta_data))
}

fn get_start_line<'a>(
    start_line: Option<&'a str>,
) -> Result<(&'a str, &'a str, &'a str), ParserError> {
    let start_line = match start_line {
        Some(val) => val,
        None => {
            return Result::Err(ParserError::InvalidMethod(Option::Some(String::from(
                "No start line",
            ))))
        }
    };

    let mut start_line = start_line.split_whitespace();

    let method: &str = start_line
        .next()
        .ok_or(ParserError::InvalidMethod(Option::Some(String::from(
            "No method",
        ))))?;
    let target: &str = start_line
        .next()
        .ok_or(ParserError::InvalidMethod(Option::Some(String::from(
            "No targert",
        ))))?;
    let version: &str = start_line
        .next()
        .ok_or(ParserError::InvalidMethod(Option::Some(String::from(
            "No HTTP version",
        ))))?;

    Result::Ok((method, target, version))
}

fn get_data<'a>(
    mut line_iter: Split<&'a str>,
    domain: &str,
) -> Result<(Option<Body>, HashMap<String, String>), ParserError> {
    let mut meta_data: HashMap<String, String> = HashMap::new();

    let mut content_type: Option<&str> = Option::None;
    let mut content_lenght: Option<&str> = Option::None;

    loop {
        let line = match line_iter.next() {
            Some(value) => value,
            None => break,
        };

        if line == "\r" || line == "" {
            break;
        }

        let (key, value) = get_key_value_pair(line)?;

        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();

        if key == "host" {
            match get_sub_domain(value, domain)? {
                Some(sub_domain) => {
                    meta_data.insert("sub-domain".to_owned(), sub_domain);
                }
                None => {}
            }
        } else if key == "content-type" {
            content_type = Option::Some(value);
        } else if key == "content-length" {
            content_lenght = Option::Some(value);
        } else {
            meta_data.insert(key, value.to_string());
        }
    }

    if content_type.is_none() || content_lenght.is_none() {
        return Result::Ok((Option::None, meta_data));
    }

    let content_type = content_type.unwrap();
    let content_lenght = content_lenght.unwrap();

    let mut body: String = String::from("");

    loop {
        let line = match line_iter.next() {
            Some(value) => value,
            None => break,
        };

        if line.contains("\u{0}") {
            body.push_str(line.trim_end_matches('\u{0}'));
            break;
        }
        body.push_str(line);
    }

    let content_type = match ContentType::new(content_type) {
        Ok(val) => val,
        Err(err) => return Result::Err(err),
    };

    let body = Body {
        content_type: content_type,
        content: body,
    };

    return Result::Ok((Option::Some(body), meta_data));
}

fn get_key_value_pair<'a>(line: &'a str) -> Result<(&'a str, &'a str), ParserError> {
    let mut line = line.split(':');
    let key = line
        .next()
        .ok_or(ParserError::InvalidMethod(Option::Some(String::from(
            "No key in key value pair",
        ))))?;
    let value = line
        .next()
        .ok_or(ParserError::InvalidMethod(Option::Some(String::from(
            "No value in key value pair",
        ))))?;

    Result::Ok((key, value))
}

fn get_sub_domain<'a>(address: &'a str, domain: &str) -> Result<Option<String>, ParserError> {
    let end: usize = match address.match_indices(domain).next() {
        Some(index) => index.0,
        None => {
            return Result::Err(ParserError::InvalidMethod(Option::Some(format!(
                "Host does not include domain({}). got {} instead",
                domain, address
            ))))
        }
    };

    if end == 0usize {
        return Result::Ok(Option::None);
    }

    return Result::Ok(Option::Some(
        address[0..end].trim_end_matches(".").to_string(),
    ));
}