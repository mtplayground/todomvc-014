use leptos::prelude::*;
use leptos::web_sys;
use types::Todo;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::api;
use crate::components::todo_footer::TodoFooter;
use crate::components::todo_item::TodoItem;

fn read_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
        .trim_start_matches("#/")
        .to_string()
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let todos = RwSignal::new(Vec::<Todo>::new());
    let new_title = RwSignal::new(String::new());
    let filter = RwSignal::new(read_hash());

    // Listen for hash changes
    let closure = Closure::wrap(Box::new(move || {
        filter.set(read_hash());
    }) as Box<dyn Fn()>);
    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback(
            "hashchange",
            closure.as_ref().unchecked_ref(),
        );
    }
    closure.forget();

    // Fetch todos on mount
    leptos::task::spawn_local(async move {
        if let Ok(fetched) = api::fetch_todos().await {
            todos.set(fetched);
        }
    });

    let filtered_todos = move || {
        let all = todos.get();
        match filter.get().as_str() {
            "active" => all.into_iter().filter(|t| !t.completed).collect::<Vec<_>>(),
            "completed" => all.into_iter().filter(|t| t.completed).collect::<Vec<_>>(),
            _ => all,
        }
    };

    let has_todos = move || !todos.get().is_empty();
    let all_completed = move || {
        let t = todos.get();
        !t.is_empty() && t.iter().all(|todo| todo.completed)
    };

    let on_toggle_all = move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(updated) = api::toggle_all().await {
                todos.set(updated);
            }
        });
    };

    let add_todo = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            let title = new_title.get_untracked().trim().to_string();
            if !title.is_empty() {
                new_title.set(String::new());
                leptos::task::spawn_local(async move {
                    if let Ok(todo) = api::create_todo(&title).await {
                        todos.update(|t| t.push(todo));
                    }
                });
            }
        }
    };

    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    autofocus=true
                    prop:value=move || new_title.get()
                    on:input=move |ev| new_title.set(event_target_value(&ev))
                    on:keydown=add_todo
                />
            </header>
            {move || {
                if has_todos() {
                    Some(view! {
                        <section class="main">
                            <input
                                id="toggle-all"
                                class="toggle-all"
                                type="checkbox"
                                prop:checked=all_completed
                                on:change=on_toggle_all
                            />
                            <label for="toggle-all">"Mark all as complete"</label>
                            <ul class="todo-list">
                                <For
                                    each=filtered_todos
                                    key=|todo| todo.id
                                    children=move |todo: Todo| {
                                        view! {
                                            <TodoItem todo=todo todos=todos />
                                        }
                                    }
                                />
                            </ul>
                        </section>
                        <TodoFooter todos=todos filter=filter />
                    })
                } else {
                    None
                }
            }}
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos"</p>
            <p>"Part of "<a href="http://todomvc.com">"TodoMVC"</a></p>
        </footer>
    }
}
