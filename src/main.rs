use warp::{path, fs::dir, Filter};

#[tokio::main]
async fn main() {
    // XXX Returns 404 on missing content, but no body.
    let content = path!().and(dir("assets"));
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("webhello: serving {}", address);
    warp::serve(content).run(address).await;
}
