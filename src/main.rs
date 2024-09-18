use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
type HEADERS = HashMap<String, Vec<String>>;
//manage methods
enum HTTPmethods {
    GET,
    POST,
    PUT,
    DELETE,
    UNKNOWN, // for other methods
}
struct Request {
    method: HTTPmethods,
    resource: String,
    headers: HEADERS,
    body: Vec<u8>,
}

struct Response {
    status_line: String,
    headers: HEADERS,
    body: Vec<u8>,
}

impl Request {
    fn new(method: HTTPmethods, resource: String, headers: HEADERS, body: Vec<u8>) -> Self {
        Request {
            method,
            resource,
            headers,
            body,
        }
    }
}

impl Response {
    fn new(status_line: String, content_type: &str, body: Vec<u8>) -> Self {
        let mut headers: HEADERS = HEADERS::new();
        headers.insert("Content-Type".to_string(), vec![content_type.to_string()]);
        headers.insert("Content-Length".to_string(), vec![body.len().to_string()]);
        headers.insert("Connection".to_string(), vec!["close".to_string()]);

        Response {
            status_line,
            headers,
            body,
        }
    }
    fn send(&self, stream: &mut TcpStream) {
        let response = format!(
            "{}\r\n{}\r\n\r\n",
            self.status_line,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.join(", ")))
                .collect::<Vec<String>>()
                .join("\r\n")
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&self.body).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap(); // Close the connection
    }
}

fn parse_request(stream: &mut TcpStream) -> Request {
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    //read in String
    let request = String::from_utf8_lossy(&buffer[..bytes]);
    //split with /r/n
    let mut lines = request.lines();
    let method_line = lines.next().unwrap_or("");
    //read methods and resources
    let (method, resource) = {
        let parts: Vec<&str> = method_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let method = match parts[0] {
                "GET" => HTTPmethods::GET,
                "POST" => HTTPmethods::POST,
                _ => HTTPmethods::UNKNOWN,
            };
            let resource = parts[1];
            (method, resource.to_string())
        } else {
            (HTTPmethods::UNKNOWN, "/".to_string())
        }
    };
    let mut headers = HEADERS::new();
    for line in lines {
        if let Some((key, val)) = line.split_once(": ") {
            headers
                .entry(key.to_string())
                .or_insert(vec![])
                .push(val.to_string());
        }
    }
    //return request
    Request::new(method, resource, headers, vec![])
}

fn get_content_type(resource: &str) -> &str {
    if resource.ends_with(".html") {
        "text/html"
    } else if resource.ends_with(".css") {
        "text/css"
    } else if resource.ends_with(".js") {
        "application/javascript"
    } else if resource.ends_with(".png") {
        "image/png"
    } else if resource.ends_with(".jpg") {
        "image/jpeg"
    } else if resource.ends_with(".ico") {
        "image/x-icon"
    } else {
        "application/octet-stream"
    }
}

fn get_file(resource: &str) -> Vec<u8> {
    let path = format!("static{}", resource);
    match fs::read(&path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("failed to read: {}", path);
            vec![]
        }
    }
}

fn get_response(request: &Request) -> Response {
    match request.method {
        HTTPmethods::GET => {
            let resource = request.resource.as_str();
            //get content_type
            let content_type = get_content_type(resource);
            //get content
            let body = get_file(resource);
            if body.is_empty() {
                Response::new(
                    "HTTP/1.1 404 Not Found".to_string(),
                    "text/html",
                    b"<h1> 404 Not Found".to_vec(),
                )
            } else {
                Response::new("HTTP/1.1 200 OK".to_string(), content_type, body)
            }
        }
        _ => Response::new(
            "HTTP/1.1 405 Method Not allowed".to_string(),
            "text/html",
            b"<h1>Method not allowed</h1>".to_vec(),
        ),
    }
}

fn handle_client(mut stream: TcpStream) {
    let request = parse_request(&mut stream); //get request parsed
    let response = get_response(&request); //get_response
    response.send(&mut stream); //send response
}
fn main() {
    println!("Waiting for client to connect");
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
