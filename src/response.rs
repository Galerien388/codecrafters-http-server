use crate::headers::Header;

pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

pub struct Response {
    code: StatusCode,
    headers: Vec<Header>,
}
