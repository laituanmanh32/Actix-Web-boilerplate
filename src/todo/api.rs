use actix_web::{get, put, HttpResponse, post, web};
use actix_web::Responder;

use crate::api_error::ApiError;
use crate::todo::models::{CreateTodoMessage, Todo, UpdateTodoMessage};

#[get("/todos")]
pub async fn list() -> Result<HttpResponse, ApiError> {
    let todos = Todo::find_all()?;
    Ok(HttpResponse::Ok().json(
        todos
    ))
}

#[get("/todos/{id}")]
pub async fn find(id: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let todo = Todo::find_by_id(id.into_inner())?;
    Ok(HttpResponse::Ok().json(
        todo
    ))
}

#[post("/todos")]
pub async fn create(todo: web::Json<CreateTodoMessage>) -> Result<HttpResponse, ApiError> {
    let todo = Todo::create(todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[put("/todos/{id}")]
pub async fn update(id: web::Path<i64>, update: web::Json<UpdateTodoMessage>) -> Result<HttpResponse, ApiError> {
    let todo = Todo::update(id.into_inner(), update.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}