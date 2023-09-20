use crate::http::{method::Method, request::Request, response::Response, status::HTTPResponseCode, handler::Handler};
use std::fs;

pub struct WebServe {
    uri: String,
}

impl WebServe {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }

    fn read_default_site(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.uri, file_path);
        // Return the canonical, absolute form of a path with all
        // intermediate components normalized and symbolic links resolved.
        if let Ok(path) = fs::canonicalize(path) {
            if path.starts_with(&self.uri) {
                fs::read_to_string(path).ok()
            } else {
                eprintln!("Attack Attempted: {} Suspected attack: directory traversal", file_path);
                None
            }
        } else {
            None
        }
    }
}

impl Handler for WebServe {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(HTTPResponseCode::Ok, self.read_default_site("index.html")),
                "/hello" => Response::new(HTTPResponseCode::Ok, self.read_default_site("hello.html")),
                path => match self.read_default_site(path) {
                    Some(contents) => Response::new(HTTPResponseCode::Ok, Some(contents)),
                    None => Response::new(HTTPResponseCode::NotFound, None),
                },
            },
            _ => Response::new(HTTPResponseCode::NotFound, None),
        }
    }
}
