use crate::headers::Header;

pub fn echo_handler(echo: &str) -> Vec<u8> {
    let body = echo.as_bytes();
    let mut header = Header::new();
    header.add_header("Content-Type", "text/plain");
    header.add_header("Content-Length", body.len().to_string());
    let mut response = header.to_vec();
    response.extend(body);
    response
}
