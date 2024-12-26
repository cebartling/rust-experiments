use crate::components::tomato_icon::TomatoIcon;
use leptos::prelude::*;

#[component]
pub fn CompletedPomodoros(
    /// The number of completed pomodoros to display
    #[prop(into)]
    count: Signal<i32>,
) -> impl IntoView {
    view! {
        <div class="mb-4">
            <h2 class="text-xl font-semibold mb-2">"Completed Pomodoros"</h2>
            <div class="flex flex-wrap justify-center gap-2 transition-opacity ease-in-out delay-500">
                {move || { (0..count.get()).map(|_| view! { <TomatoIcon /> }).collect::<Vec<_>>() }}
            </div>
        </div>
    }
}
