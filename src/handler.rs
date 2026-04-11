use std::{
    fs,
    io::{self, ErrorKind, Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::{
    request::Request,
    response::{self, Response, StatusCode},
};
use anyhow::{Context, Result};

pub fn echo(req: &Request) -> Result<Response> {
    let echo = req
        .path
        .strip_prefix("/echo/")
        .context("path should start with /echo/")?;

    let resp = Response::new(StatusCode::Ok);

    let resp = match req
        .get_header("accept-encoding")
        .map(|v| v.contains(&"gzip".to_string()))
        .unwrap_or(false)
    {
        true => {
            let r = resp.with_header("content-encoding", "gzip");
            println!("TRUE {:?}", r);
            r
        }
        false => {
            println!("FALSE {:?}", resp);
            resp
        }
    };

    let body = echo.as_bytes().to_vec();
    Ok(resp
        .with_header("Content-Type", "text/plain")
        .with_body(body))
}

pub fn user_agent(req: &Request) -> Result<Response> {
    let body = req
        .get_header("user-agent")
        .ok_or(anyhow::anyhow!("user-agent header is missing"))?
        .first()
        .map(|b| b.as_str())
        .unwrap_or("");

    eprint!("body: {body}");

    Ok(Response::new(response::StatusCode::Ok)
        .with_header("content-type", "text/plain")
        .with_body(body.as_bytes().to_vec()))
}

pub fn get_file(req: &Request) -> Result<Response> {
    let path_str = path_as_str(req)?;
    let path = Path::new(&path_str);

    match fs::File::open(path) {
        Ok(file) => {
            let size = file
                .metadata()
                .context("file should exists with metadata")?
                .size();
            if size >= 800_000 {
                return Ok(Response::new(StatusCode::ContentTooLarge));
            }
            let mut response = Response::new(StatusCode::Ok)
                .with_header("content-type", "application/octet-stream")
                .with_header("content-length", size.to_string());
            let mut limit_file = file.take(800_000);
            io::copy(&mut limit_file, &mut response.body)?;
            Ok(response)
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                Ok(Response::new(StatusCode::NotFound))
            } else {
                Ok(Response::new(StatusCode::InternalServerError))
            }
        }
    }
}

pub fn post_file(req: &Request) -> Result<Response> {
    let path_str = path_as_str(req)?;
    let path = Path::new(&path_str);

    match fs::File::create(path) {
        Ok(mut file) => {
            file.write_all(req.body.as_ref())
                .context("error writing body to file")?;
            Ok(Response::new(StatusCode::Created))
        }
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

fn path_as_str(req: &Request) -> Result<String> {
    let Some(dir) = std::env::args()
        .skip_while(|arg| arg.as_str() != "--directory")
        .nth(1)
    else {
        return Err(anyhow::anyhow!("no directory supplied on the command line"));
    };

    let file_name = req
        .path
        .strip_prefix("/files/")
        .context("path should start with /files/")?;

    Ok(format!("{}/{}", dir, file_name))
}
