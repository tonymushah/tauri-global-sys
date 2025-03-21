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
    let res = LocalResource::new(move || {
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
    let is_pending = Memo::new(move |_| res.read().is_none());
    view! {
        <div>
            <input type="text" placeholder="Url" bind:value=(url, set_url) />
            <button
                on:click=move |_| {
                    res.refetch();
                }
                disabled=is_pending
            >
                "Refetch"
            </button>
            <br />
            <article>
                {move || {
                    match res.read().as_deref() {
                        Some(Ok(value)) => view! { <FormatDebug dbg=value /> }.into_any(),
                        Some(Err(err)) => {
                            view! { <p style:color="red">{format!("{err}")}</p> }.into_any()
                        }
                        _ => ().into_any(),
                    }
                }}
            </article>
        </div>
    }
}
