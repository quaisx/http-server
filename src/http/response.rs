use std::io::{Result as IoResult, Write};

use super::status::HTTPResponseCode;

#[derive(Debug)]
pub struct Response {
    response_code: HTTPResponseCode,
    body: Option<String>,
}

impl Response {
    pub fn new(code: HTTPResponseCode, body: Option<String>) -> Self {
        Response { response_code: code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.response_code,
            self.response_code.to_string(),
            body
        )
    }
}
