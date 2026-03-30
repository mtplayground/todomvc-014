use axum::routing::get;
use axum::Router;

use crate::handlers::{self, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route(
            "/api/todos/{id}",
            get(handlers::get_todo)
                .patch(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .with_state(state)
}
