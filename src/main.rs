use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
async fn home_page() -> &'static str {
    "Hello! You have successfully reached your brand-new Rust server!"
}

