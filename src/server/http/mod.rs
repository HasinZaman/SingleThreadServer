//! http module is responsible for defining how HTTP requests are parsed and how HTTP responses should be formatted
pub mod body;
pub mod request;
pub mod response;

pub use request::method;
