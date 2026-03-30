use leptos::prelude::*;
use types::Todo;

use crate::api;

#[component]
pub fn TodoFooter(
    todos: RwSignal<Vec<Todo>>,
    filter: RwSignal<String>,
) -> impl IntoView {
    let active_count = move || todos.get().iter().filter(|t| !t.completed).count();
    let has_completed = move || todos.get().iter().any(|t| t.completed);

    let on_clear_completed = move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(remaining) = api::clear_completed().await {
                todos.set(remaining);
            }
        });
    };

    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{active_count}</strong>
                {move || if active_count() == 1 { " item left" } else { " items left" }}
            </span>
            <ul class="filters">
                <li><a class:selected=move || filter.get().is_empty() href="#/">"All"</a></li>
                <li><a class:selected=move || filter.get() == "active" href="#/active">"Active"</a></li>
                <li><a class:selected=move || filter.get() == "completed" href="#/completed">"Completed"</a></li>
            </ul>
            {move || {
                if has_completed() {
                    Some(view! {
                        <button class="clear-completed" on:click=on_clear_completed>
                            "Clear completed"
                        </button>
                    })
                } else {
                    None
                }
            }}
        </footer>
    }
}
