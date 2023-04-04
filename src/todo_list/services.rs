use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodoListEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[get("/todolist/entries")]
async fn get_entries(app_state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TodoListEntry,
        r#"SELECT id, title AS "title!", date AS "date!" FROM todolist"#
    )
    .fetch_all(&app_state.db)
    .await
    {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/todolist/entries")]
async fn create_entry(
    app_state: web::Data<AppState>,
    entry_data: web::Json<CreateEntryData>,
) -> impl Responder {
    match sqlx::query!(
        r#"INSERT INTO todolist (title, date) VALUES ($1, $2) RETURNING id"#,
        entry_data.title,
        entry_data.date
    )
    .fetch_one(&app_state.db)
    .await
    {
        Ok(entry) => {
            let entry = TodoListEntry {
                id: entry.id,
                title: entry_data.title.clone(),
                date: entry_data.date,
            };
            HttpResponse::Ok().json(entry)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/todolist/entries/{id}")]
async fn update_entry(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    entry_data: web::Json<UpdateEntryData>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query!(
        r#"UPDATE todolist SET title = $1 WHERE id = $2 RETURNING id, title, date"#,
        entry_data.title,
        id
    )
    .fetch_one(&app_state.db)
    .await
    {
        Ok(entry) => {
            let entry = TodoListEntry {
                id: entry.id,
                title: entry.title.unwrap_or("".to_string()),
                date: entry.date.unwrap_or(0),
            };
            HttpResponse::Ok().json(entry)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query!("DELETE FROM todolist WHERE id = $1", id)
        .execute(&app_state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
