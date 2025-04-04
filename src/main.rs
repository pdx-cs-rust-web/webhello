// Thanks to Google Search Labs AI for DNS lookup code.
use std::net::{TcpStream, ToSocketAddrs};
use std::fs::File;
use std::io::{Write, read_to_string};

fn fetch_page() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:3000".to_socket_addrs()?.next().unwrap();
    let mut w = TcpStream::connect(socket_addr)?;
    write!(w, "GET / HTTP/1.0\r\n\r\n")?;
    let response = read_to_string(&w)?;
    let header_posn = response.find("\r\n\r\n").unwrap();
    let response = &response[header_posn + 4..];
    let mut f = File::create("response.html")?;
    write!(f, "{}", response)?;
    Ok(())
}

fn main() {
    fetch_page().unwrap();
}
