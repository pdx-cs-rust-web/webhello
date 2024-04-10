use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{BufRead, Write, BufReader};
use std::path::Path;

#[derive(Debug)]
#[allow(dead_code)]
enum ServerError {
    IoError(std::io::Error),
    HeaderFormat(String),
}
use ServerError::*;

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        IoError(e)
    }
}

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    _headers: Vec<(String, String)>,
}

fn get_request(tcp_stream: &mut TcpStream) -> Result<Request, ServerError> {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    let method = fields[0].into();
    let path = fields[1].into();

    let mut headers = Vec::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        let line = line.trim_end();
        if line.is_empty() {
            let result = Request { method, path, _headers: headers };
            return Ok(result);
        }
        let (name, value) = line
            .split_once(": ")
            .ok_or_else(|| HeaderFormat(line.into()))?;
        headers.push((name.into(), value.into()));
    }
}

fn send_content<P: AsRef<Path>>(tcp_stream: &mut TcpStream, path: P) -> Result<(), ServerError> {
    use std::io::Read;

    let mut content = Vec::new();
    std::fs::File::open(path)?.read_to_end(&mut content)?;

    tcp_stream.write_all(&content)?;

    Ok(())
}

fn send_lines(tcp_stream: &mut TcpStream, lines: &[&str]) -> Result<(), ServerError> {
    for line in lines {
        write!(tcp_stream, "{}\r\n", line)?;
    }
    Ok(())
}

fn send_page(tcp_stream: &mut TcpStream) -> Result<(), ServerError> {
    send_lines(tcp_stream, &[
        "HTTP/1.0 200 OK",
        "Content-Type: text/html; charset=utf-8",
        "",
    ])?;
    send_content(tcp_stream, "assets/index.html")
}

fn send_favicon(tcp_stream: &mut TcpStream) -> Result<(), ServerError> {
    send_lines(tcp_stream, &[
        "HTTP/1.0 200 OK",
        "Content-Type: image/vnd.microsoft.icon",
        "",
    ])?;
    send_content(tcp_stream, "assets/favicon.ico")
}

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    eprintln!("server starts: {}", socket_addr);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        let request = get_request(&mut tcp_stream).unwrap();
        eprintln!("{}: {} {}", addr, request.method, request.path);
        if request.method != "GET" {
            write!(tcp_stream, "405 Method Not Allowed\r\n").unwrap();
        } else {
            match request.path.as_ref() {
                "/" => send_page(&mut tcp_stream).unwrap(),
                "/favicon.ico" => send_favicon(&mut tcp_stream).unwrap(),
                _ => write!(tcp_stream, "404 Not Found\r\n").unwrap(),
            }
        }
        tcp_stream.flush().unwrap();
    }
}
