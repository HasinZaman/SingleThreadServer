use std::convert::AsRef;
use strum_macros::AsRefStr;

/// ResponseStatusCode is a enum of all HTTP [response status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
/// 
/// Variants between 100 - 199 relate to Information codes
/// Variants between 200 - 299 relate to Successful codes
/// Variants between 300 - 399 relate to Redirection codes
/// Variants between 400 - 499 relate to Client Error codes
/// Variants between 500 - 599 relate to Server Error codes
#[allow(dead_code)]
#[derive(Debug, AsRefStr, Clone)]
pub enum ResponseStatusCode {
    //Information
    Continue = 100,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    //Successful
    Ok = 200,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    //Redirection
    MultipleChoice = 300,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect = 307,
    PermanentRedirect,
    //ClientError
    BadRequest = 400,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest = 421,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired = 428,
    TooManyRequests,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    //ServerError
    InternalServerError = 500,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended = 510,
    NetworkAuthenticationRequired,
}

impl ResponseStatusCode {
    /// get_code method gets the code number of enum variant
    pub fn get_code(&self) -> u16 {
        self.clone() as u16
    }
}

impl ToString for ResponseStatusCode {
    fn to_string(&self) -> String {
        format!("{} {}", self.get_code(), split_at_capital(self.as_ref()))
    }
}

/// split_at_capital returns a string of ResponseStatusCode variant with spaces between upper case letters
fn split_at_capital(input: &str) -> String {
    let mut tmp = String::from("");

    let mut iter = input.chars();

    //adds first character without adding space in front of char
    tmp.push(iter.next().unwrap());

    for c in iter {
        if c.is_uppercase() {
            tmp.push(' ');
        }
        tmp.push(c)
    }

    tmp
}
