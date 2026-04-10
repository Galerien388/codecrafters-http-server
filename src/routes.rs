use crate::{
    handler,
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};
use anyhow::Result;

pub fn router(request: &Request) -> Result<Response> {
    match request.path.as_str() {
        "/" => Ok(Response::new(StatusCode::Ok)),
        _ if request.path.starts_with("/echo/") => Ok(handler::echo(request)?),
        _ if request.path.starts_with("/user-agent/") => Ok(handler::user_agent(request)?),
        _ if request.path.starts_with("/files/") => {
            if request.method == HttpMethod::Get {
                Ok(handler::get_file(request)?)
            } else if request.method == HttpMethod::Post {
                Ok(handler::post_file(request)?)
            } else {
                Ok(Response::new(StatusCode::MethodNotAllowed))
            }
        }

        _ => Ok(Response::new(StatusCode::NotFound)),
    }
}
