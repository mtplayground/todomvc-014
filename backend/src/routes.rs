use axum::routing::{delete, get};
use axum::Router;

use crate::handlers::{self, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
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
        .with_state(state)
}
