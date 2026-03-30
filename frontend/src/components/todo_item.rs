use leptos::prelude::*;
use leptos::web_sys;
use types::{Todo, UpdateTodo};

use crate::api;

fn save_edit(
    id: i64,
    editing: RwSignal<bool>,
    edit_text: RwSignal<String>,
    title: RwSignal<String>,
    todos: RwSignal<Vec<Todo>>,
) {
    if !editing.get_untracked() {
        return;
    }
    editing.set(false);
    let new_title = edit_text.get_untracked().trim().to_string();

    if new_title.is_empty() {
        leptos::task::spawn_local(async move {
            if api::delete_todo(id).await.is_ok() {
                todos.update(|t| t.retain(|item| item.id != id));
            }
        });
    } else if new_title != title.get_untracked() {
        title.set(new_title.clone());
        let update = UpdateTodo {
            title: Some(new_title),
            completed: None,
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
    }
}

#[component]
pub fn TodoItem(
    todo: Todo,
    todos: RwSignal<Vec<Todo>>,
) -> impl IntoView {
    let id = todo.id;
    let completed = RwSignal::new(todo.completed);
    let title = RwSignal::new(todo.title.clone());
    let editing = RwSignal::new(false);
    let edit_text = RwSignal::new(todo.title.clone());

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

    let start_editing = move |_| {
        edit_text.set(title.get_untracked());
        editing.set(true);
    };

    let on_edit_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => save_edit(id, editing, edit_text, title, todos),
            "Escape" => {
                editing.set(false);
                edit_text.set(title.get_untracked());
            }
            _ => {}
        }
    };

    let on_edit_blur = move |_| {
        save_edit(id, editing, edit_text, title, todos);
    };

    view! {
        <li
            class:completed=move || completed.get()
            class:editing=move || editing.get()
        >
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || completed.get()
                    on:change=on_toggle
                />
                <label on:dblclick=start_editing>{move || title.get()}</label>
                <button class="destroy" on:click=on_destroy />
            </div>
            <input
                class="edit"
                prop:value=move || edit_text.get()
                on:input=move |ev| edit_text.set(event_target_value(&ev))
                on:keydown=on_edit_keydown
                on:blur=on_edit_blur
            />
        </li>
    }
}
