use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::SqlitePool;
use types::{CreateTodo, Todo, UpdateTodo};

pub type AppState = SqlitePool;

pub async fn list_todos(State(pool): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"SELECT id, title, completed, "order" FROM todos ORDER BY "order", id"#,
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(todos))
}

pub async fn create_todo(
    State(pool): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"INSERT INTO todos (title) VALUES (?) RETURNING id, title, completed, "order""#,
    )
    .bind(&input.title)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"SELECT id, title, completed, "order" FROM todos WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(Json(todo))
}

pub async fn update_todo(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    // Build dynamic update query
    let mut set_clauses = Vec::new();
    if input.title.is_some() {
        set_clauses.push("title = ?".to_string());
    }
    if input.completed.is_some() {
        set_clauses.push("completed = ?".to_string());
    }
    if input.order.is_some() {
        set_clauses.push(r#""order" = ?"#.to_string());
    }

    if set_clauses.is_empty() {
        return get_todo(State(pool), Path(id)).await;
    }

    let query_str = format!(
        r#"UPDATE todos SET {} WHERE id = ? RETURNING id, title, completed, "order""#,
        set_clauses.join(", ")
    );

    let mut query = sqlx::query_as::<_, Todo>(&query_str);

    if let Some(ref title) = input.title {
        query = query.bind(title);
    }
    if let Some(completed) = input.completed {
        query = query.bind(completed);
    }
    if let Some(order) = input.order {
        query = query.bind(order);
    }
    query = query.bind(id);

    let todo = query
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(todo))
}

pub async fn delete_todo(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn toggle_all(State(pool): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    // If any todo is not completed, mark all as completed; otherwise mark all as not completed
    let all_completed = sqlx::query_scalar::<_, bool>(
        "SELECT COALESCE(MIN(completed), 1) FROM todos",
    )
    .fetch_one(&pool)
    .await?;

    let new_status = !all_completed;
    sqlx::query("UPDATE todos SET completed = ?")
        .bind(new_status)
        .execute(&pool)
        .await?;

    let todos = sqlx::query_as::<_, Todo>(
        r#"SELECT id, title, completed, "order" FROM todos ORDER BY "order", id"#,
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(todos))
}

pub async fn clear_completed(State(pool): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    sqlx::query("DELETE FROM todos WHERE completed = 1")
        .execute(&pool)
        .await?;

    let todos = sqlx::query_as::<_, Todo>(
        r#"SELECT id, title, completed, "order" FROM todos ORDER BY "order", id"#,
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(todos))
}

// Error handling

pub enum AppError {
    NotFound,
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Not found"}))).into_response()
            }
            AppError::Sqlx(err) => {
                eprintln!("Database error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Internal server error"})),
                )
                    .into_response()
            }
        }
    }
}
