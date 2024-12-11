mod pomodoro_timer;
mod tomato_icon;

use crate::pomodoro_timer::PomodoroTimer;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(PomodoroTimer);
}
