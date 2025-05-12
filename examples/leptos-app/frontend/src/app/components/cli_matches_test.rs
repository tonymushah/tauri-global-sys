stylance::import_crate_style!(styles, "src/app/components/cli_matches_test.module.scss");

use leptos::prelude::*;
use tauri_global_sys::cli::get_matches;

#[component]
pub fn TestCliMatches() -> impl IntoView {
    let res = LocalResource::new(|| async { get_matches().await.map_err(|e| e.to_string()) });
    let data = move || {
        res.get().as_ref().map(|e| match e {
            Ok(ok) => view! { <code>{format!("{:#?}", ok)}</code> }.into_any(),
            Err(e) => view! {
                <div class=styles::error>
                    <p>{e.clone()}</p>
                </div>
            }
            .into_any(),
        })
    };
    view! {
        <div class=styles::cli_matches_container>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {data}
                <button on:click=move |_| {
                    res.refetch();
                }>"Reload"</button>
            </Suspense>
        </div>
    }
}
