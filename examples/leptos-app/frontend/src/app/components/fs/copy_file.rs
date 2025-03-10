use leptos::prelude::*;
use tauri_global_sys::{
    dialog::{open, save, OpenDialogOptions, SaveDialogOptions},
    fs::{copy_file, BaseDirectory, FsOptions},
};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::app::{
    components::fs::fs_option::FsOptionInputs,
    utils::deser_form_data::{deser_form_data, is_on},
};

#[derive(Debug, Clone)]
pub enum ActionType {
    SetSource,
    SetDestination,
    Copy(FormData),
}

const APPEND_INPUT_NAME: &str = "_append__";
const FS_OPTION_INPUT_NAME: &str = "fs_options";

#[component]
pub fn CopyFile() -> impl IntoView {
    let (source, set_source) = signal(None::<String>);
    let (destination, set_destination) = signal(None::<String>);
    let action = Action::<_, anyhow::Result<()>, _>::new_local(move |input: &ActionType| {
        let input = input.clone();
        async move {
            match input {
                ActionType::SetSource => {
                    let src = open(Some(OpenDialogOptions {
                        title: Some("Source file".into()),
                        ..Default::default()
                    }))
                    .await
                    .map_err(|e| anyhow::Error::msg(e.to_string()))?
                    .ok_or(anyhow::anyhow!(
                        "Empty path was given! Please give proper file"
                    ))?
                    .to_vec()
                    .into_iter()
                    .next()
                    .ok_or(anyhow::anyhow!(
                        "Empty path was given! Please give a proper source file"
                    ))?;
                    set_source.set(Some(src));
                    Ok(())
                }
                ActionType::SetDestination => {
                    let dest = save(Some(SaveDialogOptions {
                        title: Some("Destination file".into()),
                        ..Default::default()
                    }))
                    .await
                    .map_err(|e| anyhow::Error::msg(e.to_string()))?
                    .ok_or(anyhow::anyhow!(
                        "Empty path was given! Please give us a proper destination file!"
                    ))?;
                    set_destination.set(Some(dest));
                    Ok(())
                }
                ActionType::Copy(form_data) => {
                    let (src, dest) = (
                        source
                            .get_untracked()
                            .ok_or(anyhow::anyhow!("Please give a proper source file!"))?,
                        destination
                            .get_untracked()
                            .ok_or(anyhow::anyhow!("Please give a proper destination file!"))?,
                    );
                    let mut options: Option<FsOptions> = deser_form_data(&form_data)
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                    if let Some(opt) = &mut options {
                        opt.append = Some(is_on(&form_data, APPEND_INPUT_NAME));
                        opt.dir = form_data
                            .get(FS_OPTION_INPUT_NAME)
                            .as_string()
                            .filter(|e| !e.is_empty())
                            .and_then(|e| e.parse::<u8>().ok())
                            .and_then(BaseDirectory::from_repr);
                    }
                    log::debug!("{:#?}", options);
                    copy_file(&src, &dest, options)
                        .await
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                    Ok(())
                }
            }
        }
    });
    let action_data = action.value_local();
    let is_loading = action.pending();
    view! {
        {move || {
            match action_data.read().as_ref().filter(|_| is_loading.read() == false) {
                Some(Err(e)) => view! { <p style:color="red">{format!("{e}")}</p> }.into_any(),
                Some(Ok(_)) => view! { <p style:color="green">"File copied!"</p> }.into_any(),
                None => None::<()>.into_any(),
            }
        }}
        <form on:submit=move |ev| {
            ev.prevent_default();
            let form = ev
                .target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
                .expect("Should be a form element");
            let form_data = FormData::new_with_form(&form).expect("Cannot make a form data");
            action.dispatch_local(ActionType::Copy(form_data));
        }>
            <p on:click=move |_| {
                action.dispatch_local(ActionType::SetSource);
            }>
                "Source: "
                {move || {
                    match source.get() {
                        Some(src) => src.into_any(),
                        None => view! { <i>"No path selected"</i> }.into_any(),
                    }
                }}
            </p>
            <p on:click=move |_| {
                action.dispatch_local(ActionType::SetDestination);
            }>
                "Destination: "
                {move || {
                    match destination.get() {
                        Some(src) => src.into_any(),
                        None => view! { <i>"No path selected"</i> }.into_any(),
                    }
                }}
            </p>
            <FsOptionInputs
                append_input_name=APPEND_INPUT_NAME
                base_dir_select_name=FS_OPTION_INPUT_NAME
            />
            <button type="submit">"Copy ;)))"</button>
        </form>
    }
}
