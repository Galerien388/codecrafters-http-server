use crate::{
    handler,
    request::Request,
    response::{Response, StatusCode},
};
use anyhow::Result;

pub fn router(request: &Request) -> Result<Response> {
    match request.path.as_str() {
        "/" => Ok(Response::new(StatusCode::Ok)),
        _ if request.path.starts_with("/echo/") => Ok(handler::echo(request)?),
        _ if request.path.starts_with("/user-agent/") => Ok(handler::user_agent(request)?),
        _ => Ok(Response::new(StatusCode::NotFound)),
    }
}
