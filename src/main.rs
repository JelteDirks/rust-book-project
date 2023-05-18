use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:3001";

fn main() {
    let tcp_listener = TcpListener::bind(ADDRESS);

    for stream in tcp_listener.unwrap().incoming() {
        let stream = stream.unwrap();
        println!("connected to {ADDRESS}");
    }
}
