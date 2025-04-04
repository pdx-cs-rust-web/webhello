// Thanks to Google Search Labs AI for DNS lookup code.
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Write, read_to_string};

fn main() {
    let socket_addr = "www.google.com:80".to_socket_addrs().unwrap().next().unwrap();
    let mut w = TcpStream::connect(socket_addr).unwrap();
    write!(w, "GET / HTTP/1.0\r\n\r\n").unwrap();
    let response = read_to_string(&w).unwrap();
    print!("{}", response);
}
