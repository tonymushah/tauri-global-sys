use leptos::prelude::*;
use tauri_global_sys::fs::BaseDirectory;

#[component]
pub fn BaseDirectorySelect(
    #[prop(into)] name: String,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <select name=name id=id>
            <option value="">{placeholder}</option>
            {BaseDirectory::all_variants()
                .iter()
                .map(|e| view! { <option value=*e as u8>{format!("{:?}", e)}</option> })
                .collect_view()}
        </select>
    }
}
