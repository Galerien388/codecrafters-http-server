use crate::{
    handler,
    request::Request,
    response::{Response, StatusCode},
};
use anyhow::{Context, Result};

pub fn router(request: &Request) -> Result<Response> {
    match request.path.as_str() {
        "/" => Ok(Response::new(StatusCode::Ok)),
        _ if request.path.starts_with("/echo/") => {
            let echo = request
                .path
                .strip_prefix("/echo/")
                .context("path should start with /echo/")?;
            Ok(handler::echo(echo))
        }
        _ => Ok(Response::new(StatusCode::NotFound)),
    }
}
