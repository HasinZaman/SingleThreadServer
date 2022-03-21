mod response_status_code {
    use super::super::response_status_code::ResponseStatusCode;

    #[test]
    fn generic_to_string_test() {
        let response_status_code: ResponseStatusCode = ResponseStatusCode::Ok;
        assert_eq!(response_status_code.to_string(), "200 Ok");
    }

    #[test]
    fn multi_word_to_string_test() {
        let response_status_code: ResponseStatusCode = ResponseStatusCode::MultipleChoice;
        assert_eq!(response_status_code.to_string(), "300 Multiple Choice");
    }

    #[test]
    fn non_defined_enum_to_string_test() {
        let response_status_code: ResponseStatusCode = ResponseStatusCode::Created;
        assert_eq!(response_status_code.to_string(), "201 Created");
    }
}

mod response {
    use super::super::response::Response;
    use super::super::response_status_code::ResponseStatusCode;
    use super::super::super::body::{Body, ContentType};
    use super::super::super::body::{ Text };
    use std::collections::HashMap;

    #[test]
    fn no_meta_data_and_body_test(){
        let response = Response {
            status: ResponseStatusCode::Ok,
            meta_data: HashMap::new(),
            body: None
        };

        let output:Vec<u8> = format!("HTTP/1.1 {}\r\n", ResponseStatusCode::Ok.to_string()).as_bytes().to_vec();

        assert_eq!(response.as_bytes(), output);
    }

    #[test]
    fn no_meta_data_test(){
        let response = Response {
            status: ResponseStatusCode::Ok,
            meta_data: HashMap::new(),
            body: Some(Body {
                content_type: ContentType::Text(Text::html),
                content: "<html></html>".as_bytes().to_vec()
            })
        };

        let output:Vec<u8> = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\n{}",
            ResponseStatusCode::Ok.to_string(),
            "<html></html>".as_bytes().len(),
            ContentType::Text(Text::html).to_string(),
            "<html></html>"
        ).as_bytes().to_vec();

        assert_eq!(response.as_bytes(), output);
    }

    #[test]
    fn general_test(){
        let response = Response {
            status: ResponseStatusCode::Ok,
            meta_data: HashMap::from([("Cache-Control".to_string(), "private".to_string())]),
            body: Some(Body {
                content_type: ContentType::Text(Text::html),
                content: "<html></html>".as_bytes().to_vec()
            })
        };

        let output:Vec<u8> = format!("HTTP/1.1 {}\r\nCache-Control: private\r\nContent-Length: {}\r\nContent-Type: {}\r\n\n{}",
            ResponseStatusCode::Ok.to_string(),
            "<html></html>".as_bytes().len(),
            ContentType::Text(Text::html).to_string(),
            "<html></html>"
        ).as_bytes().to_vec();

        assert_eq!(response.as_bytes(), output);
    }
}
