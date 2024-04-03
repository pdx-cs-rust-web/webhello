use std::io::Read;
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};

fn hello(mut stream: TcpStream) {
    let mut req = String::new();
    stream.read_to_string(&mut req).unwrap();
    for line in req.lines() {
        eprintln!("{}", line);
    }
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let ip = SocketAddrV4::new(ip, 3000);
    let listener = TcpListener::bind(ip).unwrap();
    for stream in listener.incoming() {
        hello(stream.unwrap());
    }
}
