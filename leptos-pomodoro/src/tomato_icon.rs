use leptos::prelude::*;

#[component]
pub fn TomatoIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6 h-6 inline-block">
            <path
                d="M12 2C12 2 14 4 14 6C14 6 16 5 16 3C16 3 18 5 18 7C18 9 16 11 12 11C8 11 6 9 6 7C6 5 8 3 8 3C8 3 10 4 10 6C10 4 12 2 12 2Z"
                fill="#4CAF50"
            />
            <circle cx="12" cy="14" r="8" fill="#F44336" />
            <circle cx="10" cy="12" r="2" fill="#FFEBEE" fill-opacity="0.5" />
        </svg>
    }
}
