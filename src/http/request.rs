use crate::http::method::Method;

/// HTTP Request
pub struct Request {
    path: String,
    query_str: Option<String>,
    method: Method,
}