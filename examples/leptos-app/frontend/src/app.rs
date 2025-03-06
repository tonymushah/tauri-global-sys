stylance::import_crate_style!(styles2, "src/app.module.scss");

pub mod components;
pub mod hooks;

use components::{
    app_module::AppModule, cli_matches_test::TestCliMatches, greet::Greet, test_event::TestEvent,
};
use leptos::prelude::*;

#[component]
fn FeatureDion(
    title: String,
    #[prop(optional)] default_toggled: bool,
    children: ChildrenFn,
    #[prop(optional)] flex_col: bool,
) -> impl IntoView {
    let (toggled, set_toggled) = signal(default_toggled);
    let children = StoredValue::new(children);

    view! {
        <section class=(styles2::flex_col, flex_col)>
            <h2 on:click=move |_| {
                set_toggled
                    .update(|toggled| {
                        *toggled = !*toggled;
                    });
            }>{title}</h2>
            <Show when=move || toggled.get()>{children.read_value()()}</Show>
        </section>
    }
}

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

            <FeatureDion title="Invoke Greet".into()>
                <Greet />
            </FeatureDion>

            <FeatureDion title="TestEvent".into() flex_col=true>
                <TestEvent />
            </FeatureDion>

            <FeatureDion title="Test cli matches".into() flex_col=true>
                <TestCliMatches />
            </FeatureDion>

            <FeatureDion title="App modules".into() flex_col=true>
                <AppModule />
            </FeatureDion>
        </main>
    }
}
