#[derive(Debug)]
pub enum ParserError {
    InvalidMethod(Option<String>),//split invalid method into more precise errors
}
