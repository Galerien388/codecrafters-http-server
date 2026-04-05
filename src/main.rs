use anyhow::{Context, Result};
use std::io::{BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{
    headers::Header,
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

mod handlers;
mod headers;
mod request;
mod response;

fn main() -> Result<()> {
    // HTTP/1.1 200 OK\r\n\r\n
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server is running on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(&stream);
                let request = Request::read_from(reader)?;
                dbg!(&request);

                let reponse = match request.path.as_str() {
                    "/" if request.path.starts_with("/echo") => {
                        let echo = request
                            .path
                            .strip_suffix("/echo/")
                            .context("we a sure it containts /echo")?;
                        let mut response = Response::new(StatusCode::Ok);
                        let body = echo.as_bytes();
                        response.with_body(body);
                        response.headers.add_header("Content-Type", "text/plain");
                        response
                            .headers
                            .add_header("Content-Lenght", format!("{}", body.len()));
                        unimplemented!()
                    }
                    "/" => Response::new(StatusCode::Ok),
                    _ => Response::new(StatusCode::NotFound),
                };

                stream.write(&reponse.to_http_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
