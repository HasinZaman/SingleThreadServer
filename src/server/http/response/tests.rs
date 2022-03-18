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
    /*
    use super::super::response::Response;
    use super::super::response_status_code::ResponseStatusCode;
    use crate::server::http::body::{value, Body, ContentType};

    #[test]
    fn no_body_test() {
        let response = Response {
            status: ResponseStatusCode::Ok,
            body: None,
        };

        assert_eq!(response.to_string(), "HTTP/1.1 200 Ok");
    }

    #[test]
    fn body_test() {
        let response = Response {
            status: ResponseStatusCode::Ok,
            body: Some(Body {
                content_type: ContentType::Text(value::Text::html),
                content: String::from("<p>New File</p>"),
            }),
        };

        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 Ok\r\nContent-Length: 15\r\nContent-Type: text/html\r\n\n<p>New File</p>"
        );
    }
    */
}
