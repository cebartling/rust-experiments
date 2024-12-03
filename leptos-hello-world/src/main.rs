use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::from("World"));

    view! {
        <div class="container mx-auto p-4 max-w-md">
            <h1 class="text-2xl font-bold mb-4">"Hello, " {name}</h1>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
                placeholder="Enter your name"
                class="p-2 border rounded"
            />
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}