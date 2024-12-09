use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Welcome to Leptos!"</h1>
            <Counter/>
        </main>
    }
}

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <button
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Count: " {count}
            </button>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
