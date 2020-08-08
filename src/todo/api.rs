use actix_web::{web, get, post, HttpResponse};
use actix_web::Responder;
use crate::todo::models::Todo;

#[get("/")]
pub async fn list() -> impl Responder {
    HttpResponse::Ok().json(
        vec![
            Todo {id: 1, title: "Task 1".to_string(), status: "new".to_string() },
            Todo {id: 2, title: "Task 2".to_string(), status: "done".to_string() },
        ]
    )
}

#[get("/todos/{id}")]
pub async fn find() -> impl Responder {
    HttpResponse::Ok().json(
        Todo {id: 1, title: "Task 1".to_string(), status: "new".to_string() }
    )
}

#[post("/todos")]
pub async fn create(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo.into_inner())
}