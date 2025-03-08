use super::styles;
use leptos::prelude::*;
use tauri_global_sys::dialog::{confirm, ConfirmDialogOptions};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::app::utils::deser_form_data::deser_form_data;

#[component]
pub fn Confirm() -> impl IntoView {
    let action = Action::<_, anyhow::Result<_>, _>::new_local(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            let options: Option<ConfirmDialogOptions> =
                deser_form_data(&form_data).unwrap_or_default();
            let is_confirmed = confirm(
                &form_data
                    .get("message")
                    .as_string()
                    .filter(|d| !d.is_empty())
                    .ok_or(anyhow::Error::msg("message input is empty or not found"))?,
                options,
            )
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
            Ok(is_confirmed)
        }
    });
    let action_value = action.value_local();
    let action_status = action.pending();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_status.read() == false) {
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                Some(Ok(true)) => {
                    view! { <p style:color="green">"The user agreed :)"</p> }.into_any()
                }
                Some(Ok(false)) => {
                    view! { <p style:color="#d56">"The user denied :("</p> }.into_any()
                }
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
                <input name="message" placeholder="Ask dialog message" type="text" />
            </div>
            <div>
                <input name="cancelLabel" placeholder="Cancel Label" type="text" />
            </div>
            <div>
                <input name="okLabel" placeholder="Ok Label" type="text" />
            </div>
            <div>
                <input name="title" placeholder="Title" type="text" />
            </div>
            <div>
                <select name="type" placeholder="Dialog type">
                    <option value="info">"Info"</option>
                    <option value="warning">"Warning"</option>
                    <option value="error">"Error"</option>
                </select>
            </div>
            <div>
                <button type="submit">"Confirm..."</button>
            </div>
        </form>
    }
}
