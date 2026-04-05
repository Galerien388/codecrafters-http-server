use anyhow::Result;
use std::str::FromStr;

pub const VERSION: &'static str = "HTTP/1.1";

#[derive(Debug, Clone, PartialEq)]
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
            _ => Err(anyhow::bail!("unknown http method {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub target: String,
    pub version: &'static str,
}

impl Request {
    pub fn new(method: HttpMethod, target: String) -> Self {
        Self {
            method,
            target,
            version: VERSION, // in this excersice the version will not change
        }
    }
}
