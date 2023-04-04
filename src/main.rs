use actix_web::{get, web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;
use std::sync::Mutex;

mod todo_list;
use todo_list::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>,
    db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "OK".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
        db: pool.clone(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
