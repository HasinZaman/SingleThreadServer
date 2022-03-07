mod response_status_code{
    use strum::IntoEnumIterator;

    macro_rules! range_test_gen {
        {$min:literal, $max:literal, $enumeration:ident} => (
            use super::super::ResponseStatusCode::$enumeration;
            for variant in $enumeration::iter() {
                let val = *(&variant) as usize;
                assert!(
                    ($min..$max).contains(&val),
                    "Value must be between {} & {} . Got {:?}:{} instead",
                    $min,
                    $max,
                    variant,
                    val
                );
            }
        )
    }

    #[test]
    fn information_range_value_test(){
        range_test_gen!(100usize, 199usize, Information);
    }

    #[test]
    fn successful_range_value_test(){
        range_test_gen!(200usize, 299usize, Successful);
    }

    #[test]
    fn redirection_range_value_test(){
        range_test_gen!(300usize, 399usize, Redirection);
    }

    #[test]
    fn client_error_range_value_test(){
        range_test_gen!(400usize, 499usize, ClientError);
    }

    #[test]
    fn server_error_range_value_test(){
        range_test_gen!(500usize, 599usize, ServerError);
    }
}