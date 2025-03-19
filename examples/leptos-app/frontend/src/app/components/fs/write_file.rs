stylance::import_crate_style!(styles, "src/app/components/fs/write_file.module.scss");

use enum_all_variants::AllVariants;
use enum_repr::EnumRepr;
use leptos::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri_global_sys::{
    dialog::{save, SaveDialogOptions},
    fs::{
        write_binary_file, write_binary_file2, write_text_file, write_text_file2,
        FsBinaryFileOption, FsOptions, FsTextFileOption,
    },
};
use web_sys::FormData;

use crate::app::utils::{deser_form_data::is_on, extract_from_data::extract_form_data};

#[EnumRepr(type = "u8", implicit = true)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    AllVariants,
    Default,
)]
enum WriteMode {
    WriteBinary = 1,
    WriteBinary2,
    #[default]
    WriteText,
    WriteText2,
}

const APPEND_INPUT_NAME: &str = "append";

const CONTENT_TEXTAREA_NAME: &str = "content";

const WRITE_MODE_SELECT_NAME: &str = "write-mode";

#[component]
pub fn WriteFile() -> impl IntoView {
    let action = Action::<_, anyhow::Result<_>, _>::new_local(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            let content = form_data
                .get(CONTENT_TEXTAREA_NAME)
                .as_string()
                .filter(|content| !content.is_empty())
                .ok_or(anyhow::format_err!("File content required"))?;
            let append = is_on(&form_data, APPEND_INPUT_NAME);
            let write_mode = form_data
                .get(WRITE_MODE_SELECT_NAME)
                .as_string()
                .and_then(|d| d.parse::<u8>().ok())
                .and_then(WriteMode::from_repr)
                .unwrap_or_default();
            let path = save(
                SaveDialogOptions {
                    title: String::from("File to save").into(),
                    ..Default::default()
                }
                .into(),
            )
            .await
            .map_err(|e| anyhow::format_err!("{e}"))?
            .ok_or(anyhow::format_err!(
                "Please select a file to save the content."
            ))?;
            let fs_file_option = FsOptions {
                append: Some(append),
                ..Default::default()
            };
            match write_mode {
                WriteMode::WriteBinary => {
                    write_binary_file(&path, content.into_bytes(), fs_file_option.into())
                        .await
                        .map_err(|e| anyhow::format_err!("{e}"))?;
                }
                WriteMode::WriteBinary2 => {
                    write_binary_file2(
                        FsBinaryFileOption {
                            contents: content.into_bytes(),
                            path: path.clone(),
                        },
                        fs_file_option.into(),
                    )
                    .await
                    .map_err(|e| anyhow::format_err!("{e}"))?;
                }
                WriteMode::WriteText => {
                    write_text_file(&path, &content, fs_file_option.into())
                        .await
                        .map_err(|e| anyhow::format_err!("{e}"))?;
                }
                WriteMode::WriteText2 => {
                    write_text_file2(
                        FsTextFileOption {
                            contents: content,
                            path: path.clone(),
                        },
                        fs_file_option.into(),
                    )
                    .await
                    .map_err(|e| anyhow::format_err!("{e}"))?;
                }
            }
            Ok(path)
        }
    });
    let action_value = action.value_local();
    let action_pending = action.pending();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                Some(Ok(path)) => {
                    view! { <p style:color="green">{format!("Writed `{path}` file!")}</p> }
                        .into_any()
                }
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                _ => ().into_any(),
            }
        }}
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                action.dispatch_local(extract_form_data(&ev));
            }
            class=styles::form
        >
            <label>"Append?? " <input type="checkbox" name=APPEND_INPUT_NAME /></label>
            <label>
                "Content" <br />
                <textarea name=CONTENT_TEXTAREA_NAME cols="30" rows="10"></textarea>
            </label>
            <label>
                "Write mode" <br />
                <select name=WRITE_MODE_SELECT_NAME>
                    {WriteMode::all_variants()
                        .iter()
                        .map(|item| {
                            view! { <option value=*item as u8>{format!("{:?}", item)}</option> }
                        })
                        .collect_view()}
                </select>
            </label>
            <div>
                <button type="submit">"Write file"</button>
            </div>
        </form>
    }
}
