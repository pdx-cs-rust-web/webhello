// https://github.com/programatik29/axum-tutorial/blob/master/tutorial/02-layout.md
// https://docs.rs/axum/latest/axum/

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
