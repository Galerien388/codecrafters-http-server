use crate::response::Response;

pub fn echo(echo: &str) -> Response {
    let body = echo.as_bytes().to_vec();
    let response = Response::new(crate::response::StatusCode::Ok);
    let response = response
        .with_header("Content-Type", "text/plain")
        .with_header("Content-Length", body.len().to_string())
        .with_body(body);
    response
}
