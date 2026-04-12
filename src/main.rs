use anyhow::Result;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufReader, Error, Write},
    net::TcpStream,
};

use crate::{request::Request, response::Response};

mod handler;
mod headers;
mod request;
mod response;
mod routes;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server is running on port 4221");

    for stream in listener.incoming() {
        std::thread::spawn(move || {
            if let Err(e) = handle_connections(stream) {
                eprint!("connection errror {e}");
            }
        });
    }
    Ok(())
}

fn handle_connections(stream: Result<TcpStream, Error>) -> Result<()> {
    let stream = stream?;
    let mut reader = BufReader::new(stream);

    loop {
        let request = Request::read_from(&mut reader)?;
        dbg!(&request);
        let response = routes::router(&request)?;
        let (response, close) = check_close_connection(&request, response);

        reader.get_mut().write_all(&response.to_http_bytes()?)?;
        if close {
            break;
        }
    }
    Ok(())
}

fn check_close_connection(req: &Request, resp: Response) -> (Response, bool) {
    if req
        .get_header_value("connection")
        .unwrap_or("")
        .eq_ignore_ascii_case("close")
    {
        (resp.with_header("connection", "close"), true)
    } else {
        (resp, false)
    }
}

// fn handle_connections(stream: Result<TcpStream, Error>) -> Result<()> {
//     let mut stream = stream?;
//
//     // reader borrow the stream
//     // this way the reader will be dropped
//     // before stream is borrow as mut in write_all method
//     // Note: is not necessary with new Rust NLL (non lexical lifetimes)
//     let request = {
//         let mut reader = BufReader::new(&stream);
//         Request::read_from(&mut reader)?
//     };
//
//     dbg!(&request);
//     let response = routes::router(&request)?;
//     stream.write_all(&response.to_http_bytes()?)?;
//     Ok(())
// }
