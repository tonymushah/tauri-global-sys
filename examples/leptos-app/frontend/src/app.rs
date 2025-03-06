pub mod components;
pub mod hooks;

use components::{greet::Greet, test_event::TestEvent};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo" />
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <section>
                <h2>"Greet"</h2>
                <Greet />
            </section>

            <section>
                <h2>"TestEvent"</h2>
                <TestEvent />
            </section>
        </main>
    }
}
