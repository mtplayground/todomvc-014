pub mod api;
pub mod app;

use app::TodoApp;

fn main() {
    leptos::mount::mount_to_body(TodoApp);
}
