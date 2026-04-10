use std::{convert::Into, fmt::Display, io::Write};

use crate::headers::Headers;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    MethodNotAllowed = 405,
    ContentTooLarge = 413,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::ContentTooLarge => 413,
            StatusCode::InternalServerError => 500,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "OK"),
            StatusCode::Created => write!(f, "Created"),
            StatusCode::NotFound => write!(f, "Not Found"),
            StatusCode::MethodNotAllowed => write!(f, "Method Not Allowed"),
            StatusCode::ContentTooLarge => write!(f, "Content To Large"),
            StatusCode::InternalServerError => write!(f, "internal server error"),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub code: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        Self {
            code,
            body: Vec::new(),
            headers: Headers::new(),
        }
    }
}

impl Response {
    pub fn to_http_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::<u8>::new();

        write!(
            buffer,
            "HTTP/1.1 {} {}\r\n",
            &self.code.as_u16(),
            &self.code
        )?;

        if !self.headers.is_empty() {
            self.headers.write_to(&mut buffer)?;
        }

        write!(buffer, "\r\n")?;

        if !self.body.is_empty() {
            buffer.write_all(self.body.as_ref())?;
        }

        Ok(buffer)
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.add_header(key, value);
        self
    }

    // !! this sets the Content-Length
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.headers
            .add_header("Content-Length", body.len().to_string());
        self.body = body;
        self
    }

    pub fn _with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    pub fn _set_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into().to_ascii_lowercase(), value);
    }
}
