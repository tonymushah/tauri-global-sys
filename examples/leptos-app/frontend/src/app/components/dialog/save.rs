use super::styles;
use leptos::prelude::*;
use tauri_global_sys::dialog::{save, DialogFilter, SaveDialogOptions};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::app::utils::deser_form_data::deser_form_data;

#[component]
pub fn Save() -> impl IntoView {
    let action = Action::<_, anyhow::Result<_>>::new_local(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            let mut options: Option<SaveDialogOptions> =
                deser_form_data(&form_data).unwrap_or_default();
            if let Some(opt) = options.as_mut() {
                opt.filters = form_data
                    .get("filterss_")
                    .as_string()
                    .filter(|e| !e.is_empty())
                    .map(|e| {
                        e.split(';')
                            .flat_map(|split| {
                                Some(DialogFilter {
                                    extensions: split
                                        .split_once(':')
                                        .map(|x| x.1)
                                        .filter(|s| !s.is_empty())
                                        .map(|e| e.split(",").map(String::from).collect())
                                        .unwrap_or_default(),
                                    name: split.split(':').next().map(String::from)?,
                                })
                            })
                            .collect::<Vec<_>>()
                    });
            }
            log::debug!("{:#?}", options);
            let res = save(options)
                .await
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;
            Ok(res)
        }
    });
    let action_value = action.value();
    let action_status = action.pending();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_status.read() == false) {
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                Some(Ok(Some(data))) => view! { <strong>{data.clone()}</strong> }.into_any(),
                Some(Ok(None)) => view! { <i>"No file selected"</i> }.into_any(),
                _ => None::<()>.into_any(),
            }
        }}
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let form = ev
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
                    .expect("Should be a form element");
                let form_data = FormData::new_with_form(&form).expect("Cannot make a form data");
                action.dispatch_local(form_data);
            }
            class=styles::form
        >
            <div>
                <input name="title" placeholder="Dialog title" type="text" />
            </div>
            <div>
                <input name="defaultPath" placeholder="Default Path" type="text" />
            </div>
            <div>
                <input
                    name="filterss_"
                    placeholder="Filter. use ; to separate each entry"
                    type="text"
                />
            </div>
            <div>
                <button type="submit">"Open..."</button>
            </div>
        </form>
    }
}
