use leptos::prelude::*;
use tauri_global_sys::fs::{create_dir, BaseDirectory, FsDirOptions};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::app::{
    components::fs::fs_dir_option::FsDirOptionInputs,
    utils::deser_form_data::{deser_form_data, is_on},
};

const RECURSIVE_INPUT_NAME: &str = "_recursive";

const BASE_DIR_SELECT_NAME: &str = "something_base_dir_dsad";

#[component]
pub fn CreateDir() -> impl IntoView {
    let action = Action::<_, anyhow::Result<()>>::new_local(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            let mut options: Option<FsDirOptions> =
                deser_form_data(&form_data).map_err(|e| anyhow::Error::msg(e.to_string()))?;
            if let Some(opt) = &mut options {
                opt.recursive = Some(is_on(&form_data, RECURSIVE_INPUT_NAME));
                opt.dir = form_data
                    .get(BASE_DIR_SELECT_NAME)
                    .as_string()
                    .filter(|e| !e.is_empty())
                    .and_then(|e| e.parse::<u8>().ok())
                    .and_then(BaseDirectory::from_repr);
            }
            log::debug!("{:#?}", options);
            create_dir(
                form_data
                    .get("path")
                    .as_string()
                    .as_ref()
                    .ok_or(anyhow::Error::msg("path input name is required"))?,
                options,
            )
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
            Ok(())
        }
    });
    let action_value = action.value();
    let action_pending = action.pending();

    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                Some(Err(e)) => view! { <p style:color="red">{format!("{e}")}</p> }.into_any(),
                Some(Ok(_)) => view! { <p style:color="green">"New dir created"</p> }.into_any(),
                _ => None::<()>.into_any(),
            }
        }}
        <form on:submit=move |ev| {
            ev.prevent_default();
            let form = ev
                .target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
                .expect("Should be a form element");
            let form_data = FormData::new_with_form(&form).expect("Cannot make a form data");
            action.dispatch_local(form_data);
        }>
            <input type="text" name="path" placeholder="Directory path" />
            <FsDirOptionInputs
                recursive_input_name=RECURSIVE_INPUT_NAME
                base_dir_select_name=BASE_DIR_SELECT_NAME
            />
            <button type="submit">"Create directory"</button>
        </form>
    }
}
