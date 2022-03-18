//! Parser_error module defines enums required to parsing errors

/// ParserError enum defines variants that represent errors that can emerge through parsing of request
#[derive(Debug)]
pub enum ParserError {
    InvalidMethod(Option<String>), //split invalid method into more precise errors
}
