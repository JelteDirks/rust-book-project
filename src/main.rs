use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:3001";

fn main() {
    let tcp_listener = TcpListener::bind(ADDRESS);

    for stream in tcp_listener.unwrap().incoming() {
        let mut stream = stream.unwrap();
        println!("connected to {ADDRESS}");

        handle_connection(&mut stream);

        let status_line = "HTTP/1.1 200 OK";
        let content = std::fs::read_to_string("index.html").expect("index read");
        let content_length = content.len();
        let headers = format!("Content-Length: {content_length}\r\n");

        let response = format!("{status_line}\r\n{headers}\r\n{content}");

        stream.write_all(response.as_bytes()).expect("response msg");
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("incoming request:");
    println!("{:#?}", http_req);
}
