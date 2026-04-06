use std::{convert::Into, fmt::Display, io::Write};

use crate::headers::Headers;
use anyhow::Result;

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
    pub header: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self {
            code,
            body: Vec::new(),
            header: Headers::new(),
        }
    }
}

impl Response {
    pub fn to_http_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::<u8>::new();

        let nr = self.code.clone() as u16;

        write!(buffer, "HTTP/1.1 {} {}\r\n", nr, &self.code)?;

        if !self.header.is_empty() {
            self.header.write_to(&mut buffer)?;
        }

        write!(buffer, "\r\n")?;

        if !self.body.is_empty() {
            buffer.write_all(self.body.as_ref())?;
        }

        Ok(buffer)
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.header.add_header(key, value);
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.header = headers;
        self
    }
}
