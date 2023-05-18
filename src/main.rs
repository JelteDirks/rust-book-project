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
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        handle_root(stream);
    } else if request_line.ends_with("HTTP/1.1") {
        handle_404(stream);
    }

}

fn handle_404(mut stream: &TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let content = std::fs::read_to_string("404.html").expect("404 read");
    let content_length = content.len();
    let headers = format!("Content-Length: {content_length}\r\n");
    let response = format!("{status_line}\r\n{headers}\r\n{content}");

    stream.write_all(response.as_bytes()).expect("response msg");
}

fn handle_root(mut stream: &TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let content = std::fs::read_to_string("index.html").expect("index read");
    let content_length = content.len();
    let headers = format!("Content-Length: {content_length}\r\n");
    let response = format!("{status_line}\r\n{headers}\r\n{content}");

    stream.write_all(response.as_bytes()).expect("response msg");
}
