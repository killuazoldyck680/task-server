use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home_page))
    .route("/task", get(get_task));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
async fn home_page() -> &'static str {
    "Hello! You have successfully reached your brand-new Rust server!"
}

async fn get_task() -> Json<Task> {
    let my_todo = Task {
        id: 1,
        title: String::from("Learn Rust Web Development"),
        completed: false,
    };

    Json(my_todo)
}

