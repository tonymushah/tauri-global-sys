use std::collections::HashMap;

use futures::{
    stream::{AbortHandle, Abortable},
    StreamExt,
};
use leptos::{prelude::*, task::spawn_local};
use tauri_global_sys::global_shortcut::{register, unregister, ShorcutHandler};
use wasm_bindgen::prelude::Closure;

const COUNTER_KEY: &str = "CommandOrControl+Shift+C";

#[component]
fn Counter() -> impl IntoView {
    let (counter, set_counter) = signal(0_u32);
    spawn_local(async move {
        let closure = Closure::new(move |_| {
            set_counter.update(|count| {
                *count += 1;
            });
        });
        if let Err(err) = register(COUNTER_KEY, &closure).await {
            log::error!("{err}");
        }
        closure.forget();
    });
    on_cleanup(|| {
        spawn_local(async move {
            if let Err(err) = unregister(COUNTER_KEY).await {
                log::error!("{err}");
            }
        });
    });
    view! {
        <p>
            "Count: " {move || { counter.get() }}
            {format!(" (Use `{}` to add the count)", COUNTER_KEY)}
        </p>
    }
}

const SHORTCUT_STREAM_KEY: &[&str] = &[
    "CommandOrControl+Shift+I",
    "CommandOrControl+Shift+P",
    "CommandOrControl+Shift+F6",
];

#[component]
fn ShortcutStream() -> impl IntoView {
    let (keys_count_map, set_keys_count_map) = signal(HashMap::<String, u32>::new());
    let (abort_handle, abort_register) = AbortHandle::new_pair();
    on_cleanup(move || {
        abort_handle.abort();
    });
    spawn_local(async move {
        let maybe_stream = ShorcutHandler::new(
            SHORTCUT_STREAM_KEY
                .iter()
                .map(|s| String::from(*s))
                .collect(),
        )
        .await;
        match maybe_stream {
            Ok(stream) => {
                let mut stream = Abortable::new(stream, abort_register);
                while let Some(key) = stream.next().await {
                    set_keys_count_map.update(|map| {
                        *map.entry(key).or_default() += 1;
                    });
                }
            }
            Err(err) => {
                log::error!("{err}");
            }
        }
    });
    view! {
        <div>
            <p>
                "Use "
                {SHORTCUT_STREAM_KEY
                    .iter()
                    .map(|d| {
                        view! {
                            <strong>{String::from(*d)}</strong>
                            " "
                        }
                    })
                    .collect_view()} " to trigger the counter"
            </p>
            <ul>
                {move || {
                    keys_count_map
                        .read()
                        .iter()
                        .map(|(key, count)| {
                            view! { <li>{format!("{key} => {count}")}</li> }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}

#[component]
pub fn GlobalShortcut() -> impl IntoView {
    view! {
        <Counter />
        <ShortcutStream />
    }
}
