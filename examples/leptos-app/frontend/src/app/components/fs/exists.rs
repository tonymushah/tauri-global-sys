use leptos::prelude::*;
use tauri_global_sys::fs::{exists, BaseDirectory, FsOptions};
use web_sys::FormData;

use crate::app::{
    components::fs::fs_option::FsOptionInputs,
    utils::{deser_form_data::is_on, extract_from_data::extract_form_data},
};

const BASE_DIR_SELECT: &str = "dir_select";

const APPEND_INPUT_NAME: &str = "append???";

const PATH_INAME: &str = "to_check";

#[component]
pub fn Exists() -> impl IntoView {
    let action = Action::<_, anyhow::Result<_>, _>::new_local(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            let options = Some(FsOptions {
                append: Some(is_on(&form_data, APPEND_INPUT_NAME)),
                dir: form_data
                    .get(BASE_DIR_SELECT)
                    .as_string()
                    .and_then(|e| e.parse::<u8>().ok())
                    .and_then(BaseDirectory::from_repr),
            });
            exists(
                &form_data
                    .get(PATH_INAME)
                    .as_string()
                    .filter(|e| !e.is_empty())
                    .ok_or(anyhow::format_err!(
                        "Invalid/missing input for {PATH_INAME}"
                    ))?,
                options,
            )
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))
        }
    });

    let action_pending = action.pending();
    let action_value = action.value_local().read_only();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                Some(Ok(true)) => {
                    view! { <p style:color="green">"The actual path exists!"</p> }.into_any()
                }
                Some(Ok(false)) => {
                    view! { <p style:color="violet">"The actual path does not exists!"</p> }
                        .into_any()
                }
                Some(Err(e)) => view! { <p style:color="red">{format!("{e}")}</p> }.into_any(),
                _ => ().into_any(),
            }
        }}
        <form on:submit=move |ev| {
            ev.prevent_default();
            let _form_data = extract_form_data(&ev);
            action.dispatch_local(_form_data);
        }>
            <input name=PATH_INAME type="text" placeholder="To check" />
            <FsOptionInputs
                append_input_name=APPEND_INPUT_NAME
                base_dir_select_name=BASE_DIR_SELECT
            />
            <button type="submit">"Check..."</button>
        </form>
    }
}
