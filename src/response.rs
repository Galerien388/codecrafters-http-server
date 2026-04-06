use std::fmt::Display;

use crate::headers::{self, Header};

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
    pub headers: Option<Header>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self {
            code,
            body: None,
            headers: None,
        }
    }
}

impl Response {
    pub fn to_http_bytes(&self) -> Vec<u8> {
        let nr = self.code.clone() as u16;
        let mut version = format!("HTTP/1.1 {} {}\r\n", nr, &self.code)
            .as_bytes()
            .to_vec();

        if let Some(ref headers) = self.headers {
            version.extend(headers.to_vec());
        }

        version.extend("\r\n".as_bytes());
        if let Some(ref body) = self.body {
            version.extend(body);
        }

        version
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_headers(mut self, headers: Header) -> Self {
        self.headers = Some(headers);
        self
    }
}
