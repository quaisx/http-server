use status::HTTPResponseCode;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::method::MethodError;
use crate::http::status;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &MethodError) -> Response {
        match e {
            MethodError::Error(err) => dbg!("bad request:{}", err)

        };
        Response::new(HTTPResponseCode::BadRequest, None)
    }
}
