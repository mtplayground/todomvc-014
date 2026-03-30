use leptos::prelude::*;
use types::{Todo, UpdateTodo};

use crate::api;

#[component]
pub fn TodoItem(
    todo: Todo,
    todos: RwSignal<Vec<Todo>>,
) -> impl IntoView {
    let id = todo.id;
    let completed = RwSignal::new(todo.completed);
    let title = todo.title.clone();

    let on_toggle = move |_| {
        let new_completed = !completed.get_untracked();
        completed.set(new_completed);
        let update = UpdateTodo {
            completed: Some(new_completed),
            title: None,
            order: None,
        };
        leptos::task::spawn_local(async move {
            if let Ok(updated) = api::update_todo(id, &update).await {
                todos.update(|t| {
                    if let Some(item) = t.iter_mut().find(|item| item.id == id) {
                        *item = updated;
                    }
                });
            }
        });
    };

    let on_destroy = move |_| {
        leptos::task::spawn_local(async move {
            if api::delete_todo(id).await.is_ok() {
                todos.update(|t| t.retain(|item| item.id != id));
            }
        });
    };

    view! {
        <li class:completed=move || completed.get()>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || completed.get()
                    on:change=on_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=on_destroy />
            </div>
        </li>
    }
}
