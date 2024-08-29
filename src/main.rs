use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    loop {
        let tcp = TcpListener::accept(&listener);
        match tcp {
            Ok((mut stream, socket)) => {
                println!("connection established over socket addr: {}", socket);
                if handle_function(&mut stream) {
                    handle_response_404(&mut stream)
                } else {
                    handle_response_200(&mut stream);
                }
            }
            Err(e) => {
                println!("Failed to establish connection due to error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn handle_function(stream: &mut TcpStream) -> bool {
    let mut buffer = [0; 512];
    let _req = stream.read(&mut buffer).unwrap();
    let req_str = String::from_utf8_lossy(&buffer);
    req_str.contains("abcdefg")
}

#[allow(unused_mut)]
fn handle_response_200(mut stream: &mut TcpStream) {
    stream.write("HTTP/1.1 200 OK\r\n\r\nHello World!".as_bytes());
}

fn handle_response_404(stream: &mut TcpStream) {
    stream.write("HTTP/1.1 404 Not Found\r\n\r\nFuck Off".as_bytes());
}

#[allow(dead_code)]
fn data_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
