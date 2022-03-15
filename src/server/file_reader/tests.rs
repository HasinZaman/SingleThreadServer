mod file_Parser {
    use super::super::parse;
    use std::path::PathBuf;

    fn create_test(url: &str, expected1: &str, expected2: Option<PathBuf>) {}

    #[test]
    fn file_does_not_exist_test() {
        assert_eq!(
            parse("\\tests\\file_does_not_exist_test.txt", "tmp"),
            Option::None
        );
    }

    #[test]
    fn no_file_name_1_test() {
        assert_eq!(
            parse("\\", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\index.html"))
        );
    }

    #[test]
    fn no_file_name_2_test() {
        assert_eq!(
            parse("\\tests\\", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_1_test() {
        assert_eq!(
            parse("\\?v=A", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_2_test() {
        assert_eq!(
            parse("\\tests\\index.html?v=A", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_3_test() {
        assert_eq!(
            parse("\\tests\\test_file.html.meta?v=A", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\test_file.html.meta"))
        );
    }

    #[test]
    fn general_file_1_test() {
        assert_eq!(
            parse("\\tests\\index.html", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn general_file_2_test() {
        assert_eq!(
            parse("\\tests\\test_file.html.meta", "tmp"),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\test_file.html.meta"))
        );
    }
}

mod file_reader {
    use super::super::{get_file_content, parse};
    use std::path::PathBuf;

    #[test]
    fn general_file_read_1_test() {
        let pathBuf: Option<PathBuf> = parse("\\tests\\index.html", "tmp");

        assert!(pathBuf.is_some());

        assert_eq!(
            get_file_content(pathBuf.unwrap().as_path()),
            Option::Some("Hello, world\r\nTest Page".to_string())
        );
    }

    #[test]
    fn general_file_read_2_test() {
        let pathBuf: Option<PathBuf> = parse("\\tests\\test_file.html.meta", "tmp");

        assert!(pathBuf.is_some());

        assert_eq!(
            get_file_content(pathBuf.unwrap().as_path()),
            Option::Some("some meta data?".to_string())
        );
    }

    #[test]
    fn general_file_read_3_test() {
        let pathBuf: Option<PathBuf> = parse("\\tests\\test_page.html", "tmp");

        assert!(pathBuf.is_some());

        assert_eq!(
            get_file_content(pathBuf.unwrap().as_path()),
            Option::Some("test_page".to_string())
        );
    }
}
