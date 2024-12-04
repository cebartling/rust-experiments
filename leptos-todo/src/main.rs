use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

#[component]
fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![]);
    let (new_todo_text, set_new_todo_text) = create_signal(String::new());
    let next_id = create_rw_signal(0);

    let add_todo = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let text = new_todo_text.get().trim().to_string();
        if !text.is_empty() {
            set_todos.update(|todos| {
                todos.push(TodoItem {
                    id: next_id.get(),
                    text,
                    completed: false,
                });
            });
            next_id.update(|id| { let _ = *id + 1; });
            set_new_todo_text.set(String::new());
        }
    };

    let toggle_todo = move |id: usize| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    let delete_todo = move |id: usize| {
        set_todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    };

    let active_count = move || {
        todos.get()
            .iter()
            .filter(|todo| !todo.completed)
            .count()
    };

    view! {
        <div class="container mx-auto max-w-md p-4">
            <h1 class="text-2xl font-bold mb-4">"Todo App"</h1>

            <form on:submit=add_todo class="mb-4">
                <div class="flex gap-2">
                    <input
                        type="text"
                        placeholder="What needs to be done?"
                        class="flex-1 px-3 py-2 border rounded"
                        prop:value=move || new_todo_text.get()
                        on:input=move |ev| set_new_todo_text.set(event_target_value(&ev))
                    />
                    <button
                        type="submit"
                        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                    >
                        "Add"
                    </button>
                </div>
            </form>

            <div class="space-y-2">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    children=move |todo| {
                        view! {
                            <div class="flex items-center gap-2 p-2 border rounded">
                                <input
                                    type="checkbox"
                                    prop:checked=todo.completed
                                    on:change=move |_| toggle_todo(todo.id)
                                />
                                <span class=move || {
                                    if todo.completed {
                                        "line-through text-gray-500"
                                    } else {
                                        "text-gray-900"
                                    }
                                }>
                                    {todo.text}
                                </span>
                                <button
                                    class="ml-auto text-red-500 hover:text-red-700"
                                    on:click=move |_| delete_todo(todo.id)
                                >
                                    "×"
                                </button>
                            </div>
                        }
                    }
                />
            </div>

            <div class="mt-4 text-sm text-gray-600">
                {move || format!("{} items left", active_count())}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <TodoApp/> });
}