use anyhow::{Context, Result};
use std::io::{BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

use std::io::BufRead;

mod headers;
mod request;
mod response;

fn main() -> Result<()> {
    // HTTP/1.1 200 OK\r\n\r\n
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(stream);
                let buf = Vec::<u8>::new();

                stream.read_to_end

                // stream
                //     .write_fmt(format_args!("HTTP/1.1 200 OK\r\n\r\n"))
                //     .context("write to stream")?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
