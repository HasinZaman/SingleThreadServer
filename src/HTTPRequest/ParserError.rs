#[derive(Debug)]
pub enum ParserError {
    InvalidMethod(Option<String>),
    FileDoesNotExist,
    NotImplemented,
}