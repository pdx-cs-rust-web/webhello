// https://github.com/programatik29/axum-tutorial/blob/master/tutorial/02-layout.md
// https://docs.rs/axum/latest/axum/

use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use axum::{routing::get, Router};

async fn hello() -> &'static str {
    eprintln!("hello");
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
