use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

/*
pub enum Option<T> {
    None,
    Some(T),
}
pub use Option::*;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
pub use Result::*;
*/

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (_tcp_stream, _) = tcp_listener.accept().unwrap();
        todo!()
    }
}
