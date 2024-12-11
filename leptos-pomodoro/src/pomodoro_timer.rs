use crate::components::completed_pomodoros::CompletedPomodoros;
use leptos::prelude::*;
use log::info;
use std::time::Duration;

// const POMODORO_TIME: i32 = 25 * 60; // 25 minutes in units of seconds
const POMODORO_TIME: i32 = 1 * 5;

#[component]
pub fn PomodoroTimer() -> impl IntoView {
    let (time_remaining, set_time_remaining) = signal(POMODORO_TIME);
    let (is_active, set_is_active) = signal(false);
    let (is_paused, set_is_paused) = signal(false);
    let (completed_pomodoros, set_completed_pomodoros) = signal(0);

    // Format remaining time as MM:SS
    let formatted_time = move || {
        let minutes = time_remaining.get() / 60;
        let seconds = time_remaining.get() % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    // Timer logic using set_interval_with_handle
    let start_timer = move |_| {
        if !is_active.get() {
            set_time_remaining.set(POMODORO_TIME);
            set_is_active.set(true);
            set_is_paused.set(false);

            let handle = set_interval_with_handle(
                move || {
                    info!("Counting down: {}", time_remaining.get());
                    if time_remaining.get() > 0 {
                        set_time_remaining.update(|t| *t -= 1);
                    } else {
                        info!("Pomodoro completed!");
                        set_is_active.set(false);
                        set_is_paused.set(false);
                        set_completed_pomodoros.update(|n| *n += 1);
                        // Could add sound notification here
                    }
                },
                Duration::from_secs(1),
            );

            if let Ok(interval_handle) = handle {
                info!("Handling timer handle");
                // Store handle to clear interval when needed
                let clear_handle = StoredValue::new(interval_handle);

                Effect::new(move |_| {
                    if !is_active.get() || time_remaining.get() == -1 {
                        clear_handle.get_value().clear();
                    }
                });
            }
        }
    };

    let pause_timer = move |_| {
        set_is_paused.set(true);
    };

    let reset_timer = move |_| {
        set_is_active.set(false);
        set_is_paused.set(false);
        set_time_remaining.set(POMODORO_TIME);
    };

    // Dynamic button text based on timer state
    let start_button_text = move || {
        if is_paused.get() {
            "Resume"
        } else {
            "Start"
        }
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
            <div class="bg-white p-8 rounded-lg shadow-lg text-center">
                <h1 class="text-4xl font-bold mb-8">"Pomodoro Timer"</h1>

                <CompletedPomodoros count=completed_pomodoros />

                <div class="text-6xl font-mono mb-8">{formatted_time}</div>

                <div class="space-x-4">
                    <button
                        class="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50"
                        on:click=start_timer
                        disabled=move || is_active.get() && !is_paused.get()
                    >
                        {start_button_text}
                    </button>

                    <button
                        class="px-6 py-2 bg-yellow-500 text-white rounded hover:bg-yellow-600 disabled:opacity-50"
                        on:click=pause_timer
                        disabled=move || !is_active.get() || is_paused.get()
                    >
                        "Pause"
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
