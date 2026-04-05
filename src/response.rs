use std::convert::Into;
use std::fmt::Display;

use crate::headers::Header;

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "OK"),
            StatusCode::NotFound => write!(f, "Not Found"),
        }
    }
}

pub struct Response {
    pub code: StatusCode,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self { code, body: None }
    }
}

impl Response {
    pub fn to_http_bytes(&self) -> Vec<u8> {
        let nr = self.code.clone() as u16;
        format!("HTTP/1.1 {} {}\r\n\r\n", nr, &self.code)
            .as_bytes()
            .to_vec()
    }

    pub fn with_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }
}
