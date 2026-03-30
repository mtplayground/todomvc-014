pub mod api;
pub mod app;
pub mod components;

use app::TodoApp;

fn main() {
    leptos::mount::mount_to_body(TodoApp);
}
