use gloo_net::http::Request;
use types::{CreateTodo, Todo, UpdateTodo};

const BASE: &str = "/api/todos";

pub async fn fetch_todos() -> Result<Vec<Todo>, gloo_net::Error> {
    let resp = Request::get(BASE).send().await?;
    resp.json().await
}

pub async fn create_todo(title: &str) -> Result<Todo, gloo_net::Error> {
    let body = CreateTodo {
        title: title.to_string(),
    };
    let resp = Request::post(BASE).json(&body)?.send().await?;
    resp.json().await
}

pub async fn update_todo(id: i64, update: &UpdateTodo) -> Result<Todo, gloo_net::Error> {
    let resp = Request::patch(&format!("{BASE}/{id}"))
        .json(update)?
        .send()
        .await?;
    resp.json().await
}

pub async fn delete_todo(id: i64) -> Result<(), gloo_net::Error> {
    Request::delete(&format!("{BASE}/{id}"))
        .send()
        .await?;
    Ok(())
}

pub async fn toggle_all() -> Result<Vec<Todo>, gloo_net::Error> {
    let resp = Request::patch(BASE).send().await?;
    resp.json().await
}

pub async fn clear_completed() -> Result<Vec<Todo>, gloo_net::Error> {
    let resp = Request::delete(&format!("{BASE}/completed"))
        .send()
        .await?;
    resp.json().await
}
