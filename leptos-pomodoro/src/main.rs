mod pomodoro_timer;

use crate::pomodoro_timer::PomodoroTimer;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-4">"Welcome to Leptos!"</h1>
            <Counter/>
        </main>
    }
}

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="p-4 bg-white rounded shadow">
            <button
                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
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

    mount_to_body(PomodoroTimer);
}
