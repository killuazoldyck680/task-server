use axum::http::StatusCode;
use axum::{Json, Router, extract::{State,Path}, routing::{get, post,patch,delete}};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use serde::{Serialize,Deserialize};

use sqlx::{sqlite::Sqlite, Row};


#[derive(Serialize, Clone, sqlx::FromRow)]
struct Task {
    id: i64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
}

type AppState = SqlitePool;

#[tokio::main]
async fn main() {

    let db_url = "sqlite://tasks.db?mode=rwc";
    let pool = SqlitePool::connect(db_url).await.expect("Failed to connect to SQLite");

    sqlx::query(
       "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
         );" 
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database table");

    let app = Router::new().route("/", get(home_page))
    .route("/task", get(get_all_task))
    .route("/task", post(create_task))
    
    .route("/task/{id}", patch(toggle_task_completion))
    .route("/task/{id}", delete(delete_task))
    .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
async fn home_page() -> &'static str {
    "Hello! You have successfully reached your brand-new Rust server!"
}

async fn get_all_task(State(pool): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode>  {
    let tasks = sqlx::query_as::<_,Task>("SELECT id, title, completed FROM tasks")
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tasks))
    
}

async fn create_task  (
    State(pool): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Task>, StatusCode> {
    let new_task = sqlx::query_as::<_, Task> (
        "INSERT INTO tasks (title, completed) VALUES ($1, FALSE) RETURNING id, title, completed"
    )
    .bind(payload.title)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(new_task))
}

async fn toggle_task_completion(State(pool): State<AppState>,
    Path(id): Path<i64>) -> Result<Json<Task>, StatusCode> {
        let updated_task = sqlx::query_as::<_,Task>(
           "UPDATE tasks SET completed = NOT completed WHERE id = $1 RETURNING id, title, completed" 
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND, // 404 if ID doesn't exist
        _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

        Ok(Json(updated_task))
    }

async fn delete_task(State(pool): State<AppState>, Path(id): Path<i64>) -> Result<String, StatusCode> {
    let db_result = sqlx::query("DELETE FROM tasks WHERE id = $1")
    .bind(id)
    .execute(&pool)
    .await;
    
    let query_result = db_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if query_result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(format!("Task {} was successfully deleted from disk database", id))
    }
}    



