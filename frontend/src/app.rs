use leptos::prelude::*;
use leptos::web_sys;
use types::Todo;

use crate::api;

#[component]
pub fn TodoApp() -> impl IntoView {
    let todos = RwSignal::new(Vec::<Todo>::new());
    let new_title = RwSignal::new(String::new());

    // Fetch todos on mount
    leptos::task::spawn_local(async move {
        if let Ok(fetched) = api::fetch_todos().await {
            todos.set(fetched);
        }
    });

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
            <section class="main">
                <ul class="todo-list">
                    <For
                        each=move || todos.get()
                        key=|todo| todo.id
                        children=move |todo: Todo| {
                            view! {
                                <li class:completed=todo.completed>
                                    <div class="view">
                                        <label>{todo.title.clone()}</label>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </section>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos"</p>
            <p>"Part of "<a href="http://todomvc.com">"TodoMVC"</a></p>
        </footer>
    }
}
