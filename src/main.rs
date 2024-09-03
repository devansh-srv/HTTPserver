use std::collections::HashMap;
use std::error::Error;
use std::io::{self, prelude::*, Bytes};
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
enum HTTPmethods {
    GET,
    PUSH,
    UPDATE,
    DELETE,
}
struct Requests {
    //versions, resources and methods
    //todo : versions
    method: HTTPmethods,
    resource: String,
    header: HashMap<String, Vec<String>>,
    body: Vec<u8>,
}
impl Requests {
    fn new(mut stream: &TcpStream) -> io::Result<Requests> {
        let mut buffer: Vec<u8> = Vec::with_capacity(4096);
        match stream.read(&mut buffer) {
            //bytes read
            Ok(bytes) => {
                println!("Bytes read:{}", bytes);
                let req = String::from_utf8_lossy(&buffer[..bytes]); //safer than other utf-8 <Cow>
                println!("Req {}", req);
            }
            Err(e) => {
                print!("Unable to read request due to {}", e);
            }
        };

        todo!()
    }
}

#[allow(dead_code)]
struct Responses;
// #[allow(unused_mut)]
// fn handle_response(mut stream: &TcpStream) {
//     let response = "HTTP/1.0 200 OK \r\r\n\nHello World";
//     let _res = stream.write(response.as_bytes()).unwrap();
//     thread::sleep(Duration::from_secs(5));
//     println!("bytes write: {}", _res);
// }
//
fn main() {
    let server: Server = Server::new("0.0.0.0:8080");
    loop {
        let tcp = TcpListener::accept(&server.listener);
        match tcp {
            Ok((mut stream, socket_address)) => {
                println!(
                    "connection established over socket addr: {}",
                    socket_address
                );
                //handle_req
            }
            Err(e) => {
                eprintln!("cannot connect due to {}", e);
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn data_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
