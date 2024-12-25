use leptos::prelude::*;

#[component]
pub fn NoCompletedPomodoros() -> impl IntoView {
    view! {
        <div class="mb-4">
            <h2 class="text-xl font-semibold mb-2">"Completed Pomodoros"</h2>
            <div class="flex flex-wrap justify-center gap-2 h-6 transition-opacity ease-in-out delay-500">
                None yet!
            </div>
        </div>
    }
}
