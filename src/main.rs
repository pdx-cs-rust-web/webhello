// https://github.com/programatik29/axum-tutorial/blob/master/tutorial/02-layout.md
// https://docs.rs/axum/latest/axum/

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::str::FromStr;

use axum::Router;
use tower_http::services as tower;

#[tokio::main]
async fn main() {
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("webhello: serving {}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();

    let mime_type = FromStr::from_str(r#"text/html; encoding="utf-8""#).unwrap();
    let page = tower::ServeFile::new_with_mime("assets/index.html", &mime_type);
    let app = Router::new().nest_service("/", page);

    axum::serve(listener, app).await.unwrap();
}
