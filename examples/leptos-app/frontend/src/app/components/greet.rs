use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use tauri_global_sys::tauri::invoke;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Greet() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = GreetArgs { name: &name };
            // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", &args).await.unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <form class="row" on:submit=greet>
            <input
                id="greet-input"
                placeholder="Enter a name..."
                prop:value=name
                on:input=update_name
            />
            <button type="submit">"Greet"</button>
        </form>
        <p>{move || greet_msg.get()}</p>
    }
}
