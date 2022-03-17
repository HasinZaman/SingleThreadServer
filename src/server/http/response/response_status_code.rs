use std::convert::AsRef;
use strum_macros::AsRefStr;

///
/// For more information about HTTP response status codes - visit https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
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
    pub fn get_code(&self) -> u16 {
        self.clone() as u16
    }
}

impl ToString for ResponseStatusCode {
    fn to_string(&self) -> String {
        format!("{} {}", self.get_code(), split_at_capital(self.as_ref()))
    }
}

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
