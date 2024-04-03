use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::io::Write;

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        eprintln!("connection from {}", addr);
        write!(tcp_stream, "HTTP/1.0 200 OK\r\n\r\n").unwrap();
        write!(tcp_stream, "<html><body><em>hello world</em></body></html>\r\n").unwrap();
        tcp_stream.flush().unwrap();
    }
}
