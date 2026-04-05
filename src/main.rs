use anyhow::{Context, Result};
use std::io::{BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use std::io::BufRead;

use crate::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

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
                let mut reader = BufReader::new(&stream);
                let mut buffer = String::new();
                let _ = reader.read_line(&mut buffer)?;
                let mut parts = buffer.splitn(3, " ");
                let method = parts
                    .next()
                    .context("first part needs to be the method")?
                    .parse::<HttpMethod>()?;
                let target = parts
                    .next()
                    .context("second parts needs to be the target")?;
                anyhow::ensure!(
                    parts.next().context("third part need to be the target")? == "HTTP/1.1\r\n",
                    "in this exercise the version can only be  HTTP/1.1"
                );

                let request = Request::new(method, target.to_string());
                dbg!(&request);

                let reponse = match request.target.as_str() {
                    "/" => Response::new(StatusCode::Ok),
                    _ => Response::new(StatusCode::NotFound),
                };

                let _ = stream.write(&reponse.to_http_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
