// Thanks to Google Search Labs AI for DNS lookup code.
use std::net::{TcpStream, ToSocketAddrs};
use std::fs::File;
use std::io::{Write, read_to_string};

fn fetch_page() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "www.google.com:80".to_socket_addrs()?.next().unwrap();
    let mut w = TcpStream::connect(socket_addr)?;
    write!(w, "GET / HTTP/1.0\r\n\r\n")?;
    let response = read_to_string(&w)?;
    let mut f = File::create("response.html")?;
    write!(f, "{}", response)?;
    Ok(())
}

fn main() {
    fetch_page().unwrap();
}
