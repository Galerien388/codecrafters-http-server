use anyhow::{Context, Result};
use std::{
    convert::AsRef,
    io::{BufRead, Read},
    str::FromStr,
};

use crate::headers::Headers;

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
            _ => anyhow::bail!("unknown http method {}", s),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: HttpMethod, path: String, version: String) -> Self {
        Self {
            method,
            path,
            version,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    pub fn read_from(reader: &mut impl BufRead) -> Result<Self> {
        // Reading the first line = Request line
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut request = Self::read_request_line(&line)?;
        let mut n = 0;

        // Reading the headers
        loop {
            line.clear();
            reader.read_line(&mut line)?;
            if line.trim_end().is_empty() {
                break;
            }
            let (key, value) = Self::read_request_header(&line)?;
            if key.eq_ignore_ascii_case("Content-Length") {
                n = value.parse()?;
            }
            Self::add_header(&mut request, key, value);
        }
        // Reading the Body
        if n != 0 {
            request.body.resize(n, 0);
            let mut limit_reader = reader.take(n as u64);
            limit_reader
                .read_exact(&mut request.body)
                .context("reading request body")?;
        }

        Ok(request)
    }

    fn read_request_line(line: &str) -> Result<Self> {
        let mut parts = line.splitn(3, " ");
        let method = parts
            .next()
            .context("should contain http method")?
            .parse::<HttpMethod>()?;
        let path = parts.next().context("should containe path")?.to_string();
        let version = parts
            .next()
            .context("should contain version")?
            .trim_end()
            .to_string();

        Ok(Self::new(method, path, version))
    }

    fn read_request_header(line: &str) -> Result<(&str, &str)> {
        let (key, value) = line
            .split_once(":")
            .context("invalid header: header should contain :")?;
        Ok((key.trim(), value.trim()))
    }

    fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into().to_ascii_lowercase();
        let value = value.into().to_ascii_lowercase();
        self.headers.add_header(key, value);
    }

    pub fn get_header(&self, key: impl AsRef<str>) -> Option<&[String]> {
        self.headers.get(key).map(|v| v.as_slice())
    }

    pub fn get_header_value(&self, key: impl AsRef<str>) -> Option<&str> {
        self.headers
            .get(key)
            .map_or_else(|| None, |v| v.first())
            .map(|v| v.as_str())
    }
}
