use std::collections::HashMap;

use futures::TryFutureExt;
use leptos::prelude::*;
use serde_json::Value;
use tauri_global_sys::{
    app::{get_name, get_version},
    http::{Client, RequestOptions},
};

use crate::app::components::format_debug::FormatDebug;

#[component]
pub fn Fetch() -> impl IntoView {
    let (url, set_url) = signal::<String>("https://api.mangadex.org/manga".into());
    let action = Action::new_local(move |_: &()| {
        let client = Client::default();
        async move {
            anyhow::Ok(
                client
                    .get::<Value>(
                        url.get_untracked(),
                        RequestOptions {
                            headers: {
                                let mut headers = HashMap::new();
                                headers.insert(
                                    String::from("User-Agent"),
                                    format!(
                                        "{}/{}",
                                        get_name()
                                            .map_err(|err| anyhow::format_err!("{err}"))
                                            .await?,
                                        get_version()
                                            .map_err(|err| anyhow::format_err!("{err}"))
                                            .await?
                                    ),
                                );
                                headers
                            },
                            ..Default::default()
                        },
                    )
                    .await?,
            )
        }
    });
    action.dispatch_local(());
    let is_pending = action.pending();
    let action_value = action.value_local().read_only();
    view! {
        <div>
            <input type="text" placeholder="Url" bind:value=(url, set_url) />
            <button
                on:click=move |_| {
                    action.dispatch_local(());
                }
                disabled=is_pending
            >
                "Refetch"
            </button>
            <br />
            <article>
                {move || {
                    match action_value.read().as_ref().filter(|_| is_pending.read() == false) {
                        Some(Ok(value)) => view! { <FormatDebug dbg=value /> }.into_any(),
                        Some(Err(err)) => {
                            view! { <p style:color="red">{format!("{err}")}</p> }.into_any()
                        }
                        _ => view! { <p>"Loading..."</p> }.into_any(),
                    }
                }}
            </article>
        </div>
    }
}
