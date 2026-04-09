use anyhow::Result;
use std::io::{BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use crate::request::Request;

mod handler;
mod headers;
mod request;
mod response;
mod routes;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server is running on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&stream);
                let request = Request::read_from(&mut reader)?;
                dbg!(&request);
                let response = routes::router(&request)?;
                stream.write_all(&response.to_http_bytes()?)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
