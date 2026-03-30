use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;
use types::{CreateTodo, Todo, UpdateTodo};

async fn setup() -> Router {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");
    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");
    backend::routes::create_router(pool)
}

async fn response_json<T: serde::de::DeserializeOwned>(
    response: axum::http::Response<Body>,
) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

fn post_json(uri: &str, body: &impl serde::Serialize) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

fn patch_json(uri: &str, body: &impl serde::Serialize) -> Request<Body> {
    Request::builder()
        .method("PATCH")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap()
}

fn patch_empty(uri: &str) -> Request<Body> {
    Request::builder()
        .method("PATCH")
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

fn delete(uri: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

// Helper: create a todo and return it
async fn create_one(app: &Router, title: &str) -> Todo {
    let body = CreateTodo {
        title: title.to_string(),
    };
    let resp = app
        .clone()
        .oneshot(post_json("/api/todos", &body))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    response_json(resp).await
}

// --- CRUD Tests ---

#[tokio::test]
async fn test_list_todos_empty() {
    let app = setup().await;
    let resp = app.oneshot(get("/api/todos")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todos: Vec<Todo> = response_json(resp).await;
    assert!(todos.is_empty());
}

#[tokio::test]
async fn test_create_todo() {
    let app = setup().await;
    let todo = create_one(&app, "Buy milk").await;
    assert_eq!(todo.title, "Buy milk");
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_list_todos_after_create() {
    let app = setup().await;
    create_one(&app, "First").await;
    create_one(&app, "Second").await;

    let resp = app.clone().oneshot(get("/api/todos")).await.unwrap();
    let todos: Vec<Todo> = response_json(resp).await;
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "First");
    assert_eq!(todos[1].title, "Second");
}

#[tokio::test]
async fn test_get_todo() {
    let app = setup().await;
    let created = create_one(&app, "Get me").await;

    let resp = app
        .clone()
        .oneshot(get(&format!("/api/todos/{}", created.id)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todo: Todo = response_json(resp).await;
    assert_eq!(todo.id, created.id);
    assert_eq!(todo.title, "Get me");
}

#[tokio::test]
async fn test_get_todo_not_found() {
    let app = setup().await;
    let resp = app.oneshot(get("/api/todos/999")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_todo_title() {
    let app = setup().await;
    let created = create_one(&app, "Old title").await;

    let update = UpdateTodo {
        title: Some("New title".to_string()),
        completed: None,
        order: None,
    };
    let resp = app
        .clone()
        .oneshot(patch_json(&format!("/api/todos/{}", created.id), &update))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todo: Todo = response_json(resp).await;
    assert_eq!(todo.title, "New title");
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_update_todo_completed() {
    let app = setup().await;
    let created = create_one(&app, "Toggle me").await;

    let update = UpdateTodo {
        title: None,
        completed: Some(true),
        order: None,
    };
    let resp = app
        .clone()
        .oneshot(patch_json(&format!("/api/todos/{}", created.id), &update))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todo: Todo = response_json(resp).await;
    assert!(todo.completed);
    assert_eq!(todo.title, "Toggle me");
}

#[tokio::test]
async fn test_update_todo_not_found() {
    let app = setup().await;
    let update = json!({"title": "Nope"});
    let resp = app
        .oneshot(patch_json("/api/todos/999", &update))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_todo() {
    let app = setup().await;
    let created = create_one(&app, "Delete me").await;

    let resp = app
        .clone()
        .oneshot(delete(&format!("/api/todos/{}", created.id)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let resp = app
        .clone()
        .oneshot(get(&format!("/api/todos/{}", created.id)))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_todo_not_found() {
    let app = setup().await;
    let resp = app.oneshot(delete("/api/todos/999")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Bulk Operation Tests ---

#[tokio::test]
async fn test_toggle_all_marks_completed() {
    let app = setup().await;
    create_one(&app, "A").await;
    create_one(&app, "B").await;

    let resp = app
        .clone()
        .oneshot(patch_empty("/api/todos"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todos: Vec<Todo> = response_json(resp).await;
    assert_eq!(todos.len(), 2);
    assert!(todos.iter().all(|t| t.completed));
}

#[tokio::test]
async fn test_toggle_all_uncompletes_when_all_done() {
    let app = setup().await;
    create_one(&app, "A").await;
    create_one(&app, "B").await;

    // First toggle: mark all completed
    app.clone()
        .oneshot(patch_empty("/api/todos"))
        .await
        .unwrap();

    // Second toggle: mark all incomplete
    let resp = app
        .clone()
        .oneshot(patch_empty("/api/todos"))
        .await
        .unwrap();
    let todos: Vec<Todo> = response_json(resp).await;
    assert!(todos.iter().all(|t| !t.completed));
}

#[tokio::test]
async fn test_clear_completed() {
    let app = setup().await;
    let a = create_one(&app, "Keep").await;
    create_one(&app, "Remove").await;

    // Mark second todo as completed
    let update = UpdateTodo {
        title: None,
        completed: Some(true),
        order: None,
    };
    app.clone()
        .oneshot(patch_json(&format!("/api/todos/{}", a.id + 1), &update))
        .await
        .unwrap();

    let resp = app
        .clone()
        .oneshot(delete("/api/todos/completed"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todos: Vec<Todo> = response_json(resp).await;
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "Keep");
}

#[tokio::test]
async fn test_clear_completed_when_none() {
    let app = setup().await;
    create_one(&app, "Active").await;

    let resp = app
        .clone()
        .oneshot(delete("/api/todos/completed"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let todos: Vec<Todo> = response_json(resp).await;
    assert_eq!(todos.len(), 1);
}
