use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{BufRead, Write, BufReader};

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
}

fn get_request(tcp_stream: &mut TcpStream) -> Request {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let fields: Vec<&str> = line.trim_end().split_whitespace().collect();
    let method = fields[0].into();
    let path = fields[1].into();

    let mut headers = Vec::new();
    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            return Request { method, path, headers };
        }
        // eprintln!("header: {:?}", line.trim_end());
        let (name, value) = line.split_once(": ").unwrap();
        headers.push((name.into(), value.into()));
    }

}

fn send_page(tcp_stream: &mut TcpStream) {
        write!(tcp_stream, "HTTP/1.0 200 OK\r\n").unwrap();
        write!(tcp_stream, "Content-Type: text/html; charset=utf-8\r\n").unwrap();
        write!(tcp_stream, "\r\n").unwrap();
        write!(tcp_stream, "<html>").unwrap();
        write!(tcp_stream, "<head>").unwrap();
        write!(tcp_stream, "<meta charset=\"UTF-8\"/>").unwrap();
        write!(tcp_stream, "<title>hello world🦀</title>").unwrap();
        write!(tcp_stream, "</head>").unwrap();
        write!(tcp_stream, "<body>").unwrap();
        write!(tcp_stream, "<em>hello world🦀</em>").unwrap();
        write!(tcp_stream, "</body>").unwrap();
        write!(tcp_stream, "</html>").unwrap();
}

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    eprintln!("server starts: {}", socket_addr);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        let request = get_request(&mut tcp_stream);
        eprintln!("{}: {} {}", addr, request.method, request.path);
        send_page(&mut tcp_stream);
        tcp_stream.flush().unwrap();
    }
}
