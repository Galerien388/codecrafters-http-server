use anyhow::{Context, Result};
use std::io::{BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::{
    handler::*,
    request::Request,
    response::{Response, StatusCode},
};

mod handler;
mod headers;
mod request;
mod response;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server is running on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(&stream);
                let request = Request::read_from(reader)?;
                dbg!(&request);

                let reponse = match request.path.as_str() {
                    _ if request.path.starts_with("/echo") => {
                        let echo = request
                            .path
                            .strip_prefix("/echo/")
                            .context("we a sure it containts /echo")?;
                        handler::echo(echo)
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
