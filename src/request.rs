use anyhow::Result;

use crate::headers::Header;
use std::str::FromStr;

pub const VERSION: &'static str = "HTTP/1.1";

pub enum HttpMethod {
    Get,
    Post,
}

impl FromStr for HttpMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            _ => Err(anyhow::Error::new("Unknown http method")),
        }
    }
}

pub struct Request {
    method: HttpMethod,
    target: String,
}
