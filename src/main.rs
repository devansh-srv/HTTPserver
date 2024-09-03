use std::collections::HashMap;
use std::error::Error;
use std::io::{self, prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{self, Duration};

//entities needed are defined below

struct Server {
    listener: TcpListener,
}
impl Server {
    fn new(socket_address: &str) -> Self {
        println!("Waiting for the client to connection");
        //binding address, match and handle error

        let listener = match TcpListener::bind(socket_address) {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("cannot bind due to {}", e);
                panic!(); //improve error handling
            }
        };
        //return server instance
        Server { listener }
    }
}
#[allow(dead_code)]
struct Client;
#[derive(Debug)]
enum HTTPmethods {
    GET,
    POST,
    PUT,
    DELETE,
}
type HEADERS = HashMap<String, Vec<String>>;
#[derive(Debug)]
struct Requests {
    //versions, resources and methods
    //todo : versions
    method: HTTPmethods,
    resource: String,
    headers: HEADERS,
    body: Vec<u8>,
}
fn read_line(stream: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut buffer: Vec<u8> = Vec::with_capacity(4096);
    //reade byte by byte
    while let Some(Ok(byte)) = stream.bytes().next() {
        if byte == b'\n' {
            //reaches the end
            if buffer.ends_with(b"\r") {
                buffer.pop();
            }
            //error -h
            let line: String = String::from_utf8(buffer).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Not a valid HTTP header")
            })?;
            return Ok(line);
        }
        buffer.push(byte);
    }
    Err(io::Error::new(
        io::ErrorKind::ConnectionAborted,
        "Client aborted early",
    ))
}
impl Requests {
    fn new(stream: &mut BufReader<TcpStream>) -> io::Result<Requests> {
        let http_metadata = read_line(stream)?;
        eprintln!("{}", http_metadata);
        let mut req_line = http_metadata.split_ascii_whitespace();
        //method
        let method = match req_line.next().unwrap() {
            "GET" => HTTPmethods::GET,
            "POST" => HTTPmethods::POST,
            "PUT" => HTTPmethods::PUT,
            "DELETE" => HTTPmethods::DELETE,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unsupported HTTP method",
                ))
            }
        };
        //resource
        let resource = req_line.next().unwrap().to_string();
        //version
        let _version = req_line.next().unwrap();
        //header
        let mut headers = HEADERS::new();
        loop {
            let line = read_line(stream)?;
            if line.is_empty() {
                break;
            }
            let mut parts = line.split(":");
            let name = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            let slot_for_value = headers.entry(name).or_insert_with(|| Vec::with_capacity(1));
            slot_for_value.push(value);
        }
        //body
        let mut body = Vec::with_capacity(65536);
        stream.read(&mut body);

        Ok(Requests {
            method,
            resource,
            headers,
            body,
        })
    }
}

#[allow(dead_code)]
struct Responses;
fn main() {
    let server: Server = Server::new("0.0.0.0:8080");
    for stream in server.listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!(
                    "connected over socket address {}",
                    stream.peer_addr().unwrap()
                );
                let mut request = BufReader::new(stream);
                match Requests::new(&mut request) {
                    Ok(request) => {
                        println!("parsed reques {:?}", request);
                    }
                    Err(e) => eprintln!("failed to parse request {}", e),
                }
            }
            Err(e) => {
                eprintln!("could not connect due to {}", e);
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn data_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
