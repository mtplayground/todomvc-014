use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    #[serde(default)]
    pub order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<i64>,
}
