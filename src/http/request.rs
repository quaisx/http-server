use crate::http::error::HandleError;
use crate::http::method::Method;
use crate::http::qstring::QString;
use std::str;

const HTTP_V1_1: &str = "HTTP/1.1";
const CH_CR: char = '\r';
const CH_SP: char = ' ';

/// HTTP Request
#[derive(Debug)]
pub struct Request<'z> {
    path: &'z str,
    method: Method,
    q_string: Option<QString<'z>>,
}

impl<'z> Request<'z> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_str(&self) -> Option<&QString> {
        self.q_string.as_ref()
    }
}

impl<'z> TryFrom<&'z [u8]> for Request<'z> {
    type Error = HandleError;

    // EXAMPLE REQUEST: GET /search?item=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'z [u8]) -> Result<Request<'z>, Self::Error> {
        let request_str = str::from_utf8(buf)?;
        // TOKEN1: METHOD...REST
        let (method, request) = get_next_token(request_str).ok_or(HandleError::ErrRequest)?;
        // TOKEN2: PATH...REST
        let (mut path, request) = get_next_token(request).ok_or(HandleError::ErrRequest)?;
        // TOKEN3: PROTOCOL...REST
        let (protocol, _) = get_next_token(request).ok_or(HandleError::ErrRequest)?;
        // Check if we are dealing with the supported HTTP protocol ver.
        if protocol != "HTTP/1.1" {
            return Err(HandleError::ErrProtocol);
        }
        // String to Enum Method conversion
        let method: Method = method.parse()?;
        // Let's analyze the query string
        let mut q_string = None;
        // query string starting point - ?
        if let Some(q) = path.find('?') {
            // query string is everything past ?
            q_string = Some(QString::from(&path[q + 1..]));
            // now that we know the end of the request: the query string
            // return the path as everything to the left of
            path = &path[..q];
        }
        // We have a ready to go request
        Ok(Self {
            path,
            method,
            q_string,
        })
    }
}

/// get_next_token - positional retrieval of request tokens
/// from the request string
///     - request a string containing the request at x positional token
///     RETURN:
///         Option<(tuple)> where tuple contains the current and the rest tokens
fn get_next_token(request: &str) -> Option<(&str, &str)> {
    // we want to work with chars
    for (idx, ch) in request.chars().enumerate() {
        // stop before the next token separated by ' ' or the end '\r'
        if ch == CH_SP || ch == CH_CR {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }
    None
}
