use crate::{headers::Header, response::Response};

pub fn echo(echo: &str) -> Response {
    let mut header = Header::new();
    let body = echo.as_bytes().to_vec();
    header.add_header("Content-Type", "text/plain");
    header.add_header("Content-Length", body.len().to_string());
    let response = Response::new(crate::response::StatusCode::Ok);
    let response = response.with_headers(header).with_body(body);
    response
}
