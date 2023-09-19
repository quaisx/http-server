pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        dbg!("bad request:{}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
