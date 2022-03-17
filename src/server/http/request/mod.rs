//! request module is responsible for enums, structs and functions responsible for parsing Requests
pub mod method;
pub mod parser_error;
pub mod request;

#[cfg(test)]
mod tests;

pub use request::parse;