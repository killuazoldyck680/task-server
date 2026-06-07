use axum::http::StatusCode;
use axum::{Json, Router, extract::{State,Path}, routing::{get, post,patch,delete}};
use std::net::SocketAddr;
use serde::{Serialize,Deserialize};
use std::sync::{Arc, Mutex};


#[derive(Serialize, Clone)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
}

type AppState = Arc<Mutex<Vec<Task>>>;

#[tokio::main]
async fn main() {

    let shared_state: AppState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new().route("/", get(home_page))
    .route("/task", get(get_all_task))
    .route("/task", post(create_task))
    
    .route("/task/{id}", patch(toggle_task_completion))
    .route("/task/{id}", delete(delete_task))
    .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
async fn home_page() -> &'static str {
    "Hello! You have successfully reached your brand-new Rust server!"
}

async fn get_all_task(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}

async fn create_task  (
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Json<Task> {
    let mut tasks = state.lock().unwrap();

    let new_id = (tasks.len() + 1) as u64;

    let new_task = Task {
        id: new_id,
        title: payload.title,
        completed: false,
    };

    tasks.push(new_task.clone());

    Json(new_task)
}

async fn toggle_task_completion(State(state): State<AppState>,
    Path(id): Path<u64>) -> Result<Json<Task>, StatusCode> {
        let mut tasks = state.lock().unwrap();

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            Ok(Json(task.clone()))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

async fn delete_task(State(state): State<AppState>, Path(id): Path<u64>) -> Result<String, StatusCode> {
    let mut tasks = state.lock().unwrap();

    let initial_len = tasks.len();

    tasks.retain(|task| task.id != id);

    if tasks.len() < initial_len {
       Ok(format!("Task {} was successfully deleted", id)) 
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}    



