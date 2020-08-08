use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub status: String,
}