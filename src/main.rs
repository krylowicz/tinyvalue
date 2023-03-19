mod server;
mod cli;
mod database;

use std::net::TcpListener;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = cli::parse_flags(&args).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        server::handle_connection(stream);
    }
}
