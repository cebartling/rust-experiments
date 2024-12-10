use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn PomodoroTimer() -> impl IntoView {
    let (time_remaining, set_time_remaining) = signal(1500); // 25 minutes in seconds
    let (is_active, set_is_active) = signal(false);

    // Format remaining time as MM:SS
    let formatted_time = move || {
        let minutes = time_remaining.get() / 60;
        let seconds = time_remaining.get() % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    // Timer logic using set_interval_with_handle
    let start_timer = move |_| {
        if !is_active.get() {
            set_is_active.set(true);

            let handle = set_interval_with_handle(
                move || {
                    if time_remaining.get() > 0 {
                        set_time_remaining.update(|t| *t -= 1);
                    } else {
                        set_is_active.set(false);
                        // Could add sound notification here
                    }
                },
                Duration::from_secs(1),
            );

            if let Ok(interval_handle) = handle {
                // Store handle to clear interval when needed
                let clear_handle = StoredValue::new(interval_handle);

                Effect::new(move |_| {
                    if !is_active.get() || time_remaining.get() == 0 {
                        clear_handle.get_value().clear();
                    }
                });
            }
        }
    };

    let reset_timer = move |_| {
        set_is_active.set(false);
        set_time_remaining.set(1500);
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
            <div class="bg-white p-8 rounded-lg shadow-lg text-center">
                <h1 class="text-4xl font-bold mb-8">"Pomodoro Timer"</h1>

                <div class="text-6xl font-mono mb-8">
                    {formatted_time}
                </div>

                <div class="space-x-4">
                    <button
                        class="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50"
                        on:click=start_timer
                        disabled=move || is_active.get()
                    >
                        "Start"
                    </button>

                    <button
                        class="px-6 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                        on:click=reset_timer
                    >
                        "Reset"
                    </button>
                </div>
            </div>
        </div>
    }
}
