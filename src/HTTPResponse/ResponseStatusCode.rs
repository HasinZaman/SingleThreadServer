use strum_macros::EnumIter;

///
/// For more infomation about HTTP reponse status codes - visit https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(Debug, EnumIter)]
pub enum Response{
    Information(Information),
    Successful(Successful),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError),
}


#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Information{
    Continue = 100,
    SwitchingProtocols,
    Processing,
    EarlyHints,
}
impl Default for Information {
    fn default() -> Self { Information::Continue }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Successful{
    Ok = 200,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
}
impl Default for Successful {
    fn default() -> Self { Successful::Ok }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Redirection{
    MultipleChoice = 300,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect = 307,
    PermanentRedirect,
}
impl Default for Redirection {
    fn default() -> Self { Redirection::MultipleChoice }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum ClientError{
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
}
impl Default for ClientError {
    fn default() -> Self { ClientError::BadRequest }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum ServerError{
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
impl Default for ServerError {
    fn default() -> Self { ServerError::InternalServerError }
}