use anyhow::{Context, Result};
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() -> Result<()> {
    // HTTP/1.1 200 OK\r\n\r\n
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream
                    .write_fmt(format_args!("HTTP/1.1 200 OK\r\n\r\n"))
                    .context("write to stream")?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
