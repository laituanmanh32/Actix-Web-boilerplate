use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::db;
use crate::schema::todo;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "todo"]
pub struct CreateTodoMessage {
    pub title: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="todo"]
pub struct UpdateTodoMessage {
    pub title: Option<String>,
    pub status: Option<String>
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "todo"]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Todo {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let todos = todo::table.load::<Todo>(&conn)?;
        Ok(todos)
    }

    pub fn find_by_id(id: i64) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let todo = todo::table
            .filter(todo::id.eq(id))
            .first(&conn)?;
        Ok(todo)
    }

    pub fn create(createTodo: CreateTodoMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let todo = Todo::from(createTodo);
        let todo = diesel::insert_into(todo::table)
            .values(todo)
            .get_result(&conn)?;
        Ok(todo)
    }

    pub fn update(id: i64, update: UpdateTodoMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let todo = diesel::update(todo::table)
            .filter(todo::id.eq(id))
            .set(update)
            .get_result(&conn)?;
        Ok(todo)
    }

}

impl From<CreateTodoMessage> for Todo {
    fn from(todo: CreateTodoMessage) -> Self {
        Todo {
            id: db::generate_int_id(),
            title: todo.title,
            status: "new".to_string(),
            created_at: Utc::now(),
            updated_at: Option::from(Utc::now())
        }
    }
}