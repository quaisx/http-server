use std::fmt::{Display, Formatter, Result as FmtResult};

/*
Informational responses (100 – 199)
Successful responses (200 – 299)
Redirection messages (300 – 399)
Client error responses (400 – 499)
Server error responses (500 – 599)
 */

/// Some of the most popular HTTP Response codes are implemented (for simplicity)
#[derive(Copy, Clone, Debug)]
pub enum HTTPResponseCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    ISR = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}
impl Display for HTTPResponseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let human_sentence = match self {
            Self::Ok => "200 Ok",
            Self::BadRequest => "400 Bad Request",
            Self::Unauthorized => "401 Unauthorized",
            Self::Forbidden => "403 Forbidden",
            Self::NotFound => "404 Not Found",
            Self::ISR => "500 Internal Server Error",
            Self::NotImplemented => "501 Not Implemented",
            Self::BadGateway => "502 Bad Gateway",
            Self::ServiceUnavailable => "503 Service Unavailable",
        };
        write!(f, "{}", human_sentence)
    }
}