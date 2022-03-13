mod file_Parser {
    use super::super::{parse, url_to_path_buffer};
    use std::path::PathBuf;

    fn create_test(url: &str, expected1: &str, expected2: &str) {
        assert_eq!(url_to_path_buffer(url), PathBuf::from(expected1));
        assert_eq!(parse(url), PathBuf::from(expected2));
    }

    #[test]
    fn file_does_not_exist_test() {
        create_test(
            "\\tests\\file_does_not_exist_test.txt",
            "Site\\tests\\file_does_not_exist_test.txt",
            "Site\\404.html",
        );
    }

    #[test]
    fn no_file_name_1_test() {
        create_test("\\", "Site\\index.html", "Site\\index.html");
    }

    #[test]
    fn no_file_name_2_test() {
        create_test(
            "\\tests\\",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html",
        );
    }

    #[test]
    fn file_with_url_variables_1_test() {
        create_test("\\?v=A", "Site\\index.html", "Site\\index.html");
    }

    #[test]
    fn file_with_url_variables_2_test() {
        create_test(
            "\\tests\\index.html?v=A",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html",
        );
    }

    #[test]
    fn file_with_url_variables_3_test() {
        create_test(
            "\\tests\\test_file.html.meta?v=A",
            "Site\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta",
        );
    }

    #[test]
    fn general_file_1_test() {
        create_test(
            "\\tests\\index.html",
            "Site\\tests\\index.html",
            "Site\\tests\\index.html",
        );
    }

    #[test]
    fn general_file_2_test() {
        create_test(
            "\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta",
            "Site\\tests\\test_file.html.meta",
        );
    }
}

mod file_reader {
    use super::super::{get_file_content, parse};
    use std::path::PathBuf;

    #[test]
    fn general_file_read_1_test() {
        let pathBuf: PathBuf = parse("\\tests\\index.html");

        assert_eq!(
            get_file_content(pathBuf.as_path()),
            Option::Some("Hello, world\r\nTest Page".to_string())
        );
    }

    #[test]
    fn general_file_read_2_test() {
        let pathBuf: PathBuf = parse("\\tests\\test_file.html.meta");

        assert_eq!(
            get_file_content(pathBuf.as_path()),
            Option::Some("some meta data?".to_string())
        );
    }

    #[test]
    fn general_file_read_3_test() {
        let pathBuf: PathBuf = parse("\\tests\\test_page.html");

        assert_eq!(
            get_file_content(pathBuf.as_path()),
            Option::Some("test_page".to_string())
        );
    }
}
