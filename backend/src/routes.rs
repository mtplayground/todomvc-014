use axum::routing::{delete, get};
use axum::Router;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::handlers::{self, AppState};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(Any);

    let api = Router::new()
        .route(
            "/api/todos",
            get(handlers::list_todos)
                .post(handlers::create_todo)
                .patch(handlers::toggle_all),
        )
        .route("/api/todos/completed", delete(handlers::clear_completed))
        .route(
            "/api/todos/{id}",
            get(handlers::get_todo)
                .patch(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .with_state(state);

    api.layer(cors)
        .fallback_service(ServeDir::new("frontend/dist"))
}
