mod file_parser {
    use super::super::parse;
    use std::path::PathBuf;

    #[test]
    fn file_does_not_exist_test() {
        let allowed_extension = vec![String::from("txt")];
        assert_eq!(
            parse(
                "\\tests\\file_does_not_exist_test.txt",
                "tmp",
                &allowed_extension
            ),
            Option::None
        );
    }

    #[test]
    fn no_file_name_1_test() {
        let allowed_extension = vec![String::from("html")];
        assert_eq!(
            parse("\\", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\index.html"))
        );
    }

    #[test]
    fn no_file_name_2_test() {
        let allowed_extension = vec![String::from("html")];
        assert_eq!(
            parse("\\tests\\", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_1_test() {
        let allowed_extension = vec![String::from("html")];
        assert_eq!(
            parse("\\?v=A", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_2_test() {
        let allowed_extension = vec![String::from("html")];
        assert_eq!(
            parse("\\tests\\index.html?v=A", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn file_with_url_variables_3_test() {
        let allowed_extension = vec![String::from("meta")];
        assert_eq!(
            parse(
                "\\tests\\test_file.html.meta?v=A",
                "tmp",
                &allowed_extension
            ),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\test_file.html.meta"))
        );
    }

    #[test]
    fn general_file_1_test() {
        let allowed_extension = vec![String::from("html")];
        assert_eq!(
            parse("\\tests\\index.html", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\index.html"))
        );
    }

    #[test]
    fn general_file_2_test() {
        let allowed_extension = vec![String::from("meta")];
        assert_eq!(
            parse("\\tests\\test_file.html.meta", "tmp", &allowed_extension),
            Option::Some(PathBuf::from("Site\\tmp\\tests\\test_file.html.meta"))
        );
    }
}

mod file_reader {
    use super::super::{get_file_content_string, parse};
    use std::path::PathBuf;

    #[test]
    fn general_file_read_1_test() {
        let allowed_extension = vec![String::from("html")];
        let path_buf: Option<PathBuf> = parse("\\tests\\index.html", "tmp", &allowed_extension);

        assert!(path_buf.is_some());

        assert_eq!(
            get_file_content_string(path_buf.unwrap().as_path()),
            Option::Some("Hello, world\r\nTest Page".to_string())
        );
    }

    #[test]
    fn general_file_read_2_test() {
        let allowed_extension = vec![String::from("meta")];
        let path_buf: Option<PathBuf> =
            parse("\\tests\\test_file.html.meta", "tmp", &allowed_extension);

        assert!(path_buf.is_some());

        assert_eq!(
            get_file_content_string(path_buf.unwrap().as_path()),
            Option::Some("some meta data?".to_string())
        );
    }

    #[test]
    fn general_file_read_3_test() {
        let allowed_extension = vec![String::from("html")];
        let path_buf: Option<PathBuf> = parse("\\tests\\test_page.html", "tmp", &allowed_extension);

        assert!(path_buf.is_some());

        assert_eq!(
            get_file_content_string(path_buf.unwrap().as_path()),
            Option::Some("test_page".to_string())
        );
    }
}
