stylance::import_crate_style!(styles2, "src/app.module.scss");

pub mod components;
pub mod hooks;
pub mod utils;

use components::{
    app_module::AppModule, cli_matches_test::TestCliMatches, clip_board::ClipBoard, dialog::Dialog,
    fetch::Fetch, fs::Fs, global_shortcuts::GlobalShortcut, greet::Greet,
    notification::Notification, os::Os, test_event::TestEvent,
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
fn Features() -> impl IntoView {
    view! {
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

        <FeatureDion title="Clipboard".into() flex_col=true>
            <ClipBoard />
        </FeatureDion>

        <FeatureDion title="Dialog".into() flex_col=true>
            <Dialog />
        </FeatureDion>

        <FeatureDion title="File System".into() flex_col=true>
            <Fs />
        </FeatureDion>

        <FeatureDion title="Global shortcuts".into() flex_col=true>
            <GlobalShortcut />
        </FeatureDion>

        <FeatureDion title="Fetch".into() flex_col=true>
            <Fetch />
        </FeatureDion>

        <FeatureDion title="Notification".into() flex_col=true>
            <Notification />
        </FeatureDion>

        <FeatureDion title="Os".into() flex_col=true>
            <Os />
        </FeatureDion>
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

            <Features />
        </main>
    }
}
