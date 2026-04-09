use crate::{
    request::Request,
    response::{self, Response},
};
use anyhow::{Context, Result};

pub fn echo(req: &Request) -> Result<Response> {
    let echo = req
        .path
        .strip_prefix("/echo/")
        .context("path should start with /echo/")?;

    let body = echo.as_bytes().to_vec();

    Ok(Response::new(response::StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_body(body))
}

pub fn user_agent(req: &Request) -> Result<Response> {
    let body = req
        .get_header("user-agent")
        .ok_or(anyhow::anyhow!("user-agent header is missing"))?
        .first()
        .unwrap_or(&"".to_string());

    Ok(Response::new(response::StatusCode::Ok)
        .with_header("Content-Type", "text/plain")
        .with_body(body.as_bytes().to_vec()))
}
