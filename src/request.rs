use anyhow::{Context, Result};
use std::{io::BufRead, str::FromStr};

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
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn new(method: HttpMethod, path: String, version: String) -> Self {
        Self {
            method,
            path,
            version,
        }
    }

    pub fn read_from(mut reader: impl BufRead) -> Result<Self> {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        let mut parts = buffer.splitn(3, " ");
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
}
