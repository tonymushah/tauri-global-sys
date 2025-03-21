use leptos::prelude::*;
use serde_json::Value;
use tauri_global_sys::http::Client;

#[component]
pub fn Fetch() -> impl IntoView {
    let (url, set_url) = signal::<String>("https://api.mangadex.org/manga".into());
    let res = LocalResource::new(move || {
        let client = Client::default();
        async move {
            client
                .get::<Value>(url.get_untracked(), Default::default())
                .await
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
                        Some(Ok(value)) => format!("{:#?}", value).into_any(),
                        Some(Err(err)) => {
                            view! { <p style:color="res">{format!("{err}")}</p> }.into_any()
                        }
                        _ => ().into_any(),
                    }
                }}
            </article>
        </div>
    }
}
