mod response_status_code{
    use super::super::ResponseStatusCode::ResponseStatusCode;

    #[test]
    fn generic_to_string_test(){
        let response_status_code : ResponseStatusCode = ResponseStatusCode::Ok;
        assert_eq!(response_status_code.to_string(), "200 Ok");
    }

    #[test]
    fn multi_word_to_string_test(){
        let response_status_code : ResponseStatusCode = ResponseStatusCode::MultipleChoice;
        assert_eq!(response_status_code.to_string(), "300 Multiple Choice");
    }

    #[test]
    fn non_defined_enum_to_string_test(){
        let response_status_code : ResponseStatusCode = ResponseStatusCode::Created;
        assert_eq!(response_status_code.to_string(), "201 Created");
    }
}