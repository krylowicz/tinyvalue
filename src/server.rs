use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

enum Status {
    Ok,
    NotFound,
}

fn create_response(message: &str, status: Status) -> String {
    let mut response = String::new();

    match status {
        Status::Ok => {
            response.push_str("HTTP/1.1 200 OK\r\n");
        }
        Status::NotFound => {
            response.push_str("HTTP/1.1 404 NOT FOUND\r\n");
        }
    }

    let length = message.len();
    let contents = format!("Content-Length: {length}\r\n\r\n{message}");
    response.push_str(&contents);

    response
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let response = match &request_line[..] {
        "GET /get HTTP/1.1" => create_response("/get", Status::Ok),
        "GET /set HTTP/1.1" => create_response("/set", Status::Ok),
        _ => create_response("endpoint is either /get or /set", Status::NotFound)
    };

    stream.write_all(response.as_bytes()).unwrap();
}