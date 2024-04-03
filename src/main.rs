use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};

struct Request {
    request: String,
    headers: Vec<(String, String)>,
}

fn read_line<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

impl Request {

    fn new(stream: &mut TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let request = read_line(&mut reader);
        let mut headers = Vec::new();
        loop {
            let header = read_line(&mut reader);
            if header.is_empty() {
                return Self { request, headers };
            }
            let fields: Vec<&str> = header.split(": ").collect();
            if fields.len() != 2 {
                eprintln!("malformed header: {}", header);
                continue;
            }
            headers.push((fields[0].to_string(), fields[1].to_string()));
        }
    }
}

fn hello(mut stream: TcpStream) {
    let request = Request::new(&mut stream);
    eprintln!("{}", request.request);
    for (name, value) in request.headers {
        eprintln!("{}: {}", name, value);
    }
    writeln!(stream, "hello world\r\n").unwrap();
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let ip = SocketAddrV4::new(ip, 3000);
    let listener = TcpListener::bind(ip).unwrap();
    for stream in listener.incoming() {
        hello(stream.unwrap());
    }
}
