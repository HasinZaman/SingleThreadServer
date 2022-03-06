
mod file_Parser {
    use super::super::{url_to_path_buffer, parse};
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

    fn create_test(url : &str, expected1 : &str, expected2 : &str) {
        assert_eq!(url_to_path_buffer(url), PathBuf::from(expected1));
        assert_eq!(parse(url), PathBuf::from(expected2));
    }

    #[test]
    fn file_does_not_exist_test() {
        create_test(
            "\\tests\\file_does_not_exist_test.txt",
            "Site\\tests\\file_does_not_exist_test.txt",
            "Site\\404.html"
        );
    }
    
    #[test]
    fn no_file_name_test() {
        create_test(
            "\\",
            "Site\\index.html",
            "Site\\index.html"
        );

        create_test(
            "\\tests\\",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html"
        );
    }
    
    #[test]
    fn file_with_url_variables_test() {
        create_test(
            "\\?v=A",
            "Site\\index.html",
            "Site\\index.html"
        );

        create_test(
            "\\tests\\index.html?v=A",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html"
        );

        create_test(
            "\\tests\\test_file.html.meta?v=A",
            "Site\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta"
        );
    }
    
    #[test]
    fn general_file_test() {
        create_test(
            "\\tests\\index.html",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html"
        );

        create_test(
            "\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta"
        );
    }
}