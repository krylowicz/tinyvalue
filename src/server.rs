use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let endpoint = match &request_line[..] {
        "GET /get HTTP/1.1" => "/get",
        "GET /set HTTP/1.1" => "/set",
        _ => ""
    };

    println!("{endpoint}");
}