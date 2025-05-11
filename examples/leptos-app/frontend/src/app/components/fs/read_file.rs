use enum_all_variants::AllVariants;
use enum_kinds::EnumKind;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_global_sys::fs::{read_binary_file, read_text_file, BaseDirectory, FsOptions};
use web_sys::FormData;

use crate::app::{
    components::fs::fs_option::FsOptionInputs,
    utils::{deser_form_data::is_on, extract_from_data::extract_form_data},
};

#[derive(Debug, Clone, EnumKind)]
#[enum_kind(ReadActionType, derive(AllVariants, Deserialize, Serialize))]
pub enum ReadActionReturn {
    Binary(Vec<u8>),
    Text(String),
}

const FILE_PATH_INAME: &str = "file_path";

const APPEND_INAME: &str = "append??";

const BASE_DIR_INAME: &str = "base_dir??";

const ACTION_TYPE_SELECT: &str = "action_type";

#[component]
pub fn ReadFile() -> impl IntoView {
    let action =
        Action::<_, anyhow::Result<ReadActionReturn>>::new_local(|form_data: &FormData| {
            let form_data = form_data.clone();
            async move {
                let options = Some(FsOptions {
                    append: Some(is_on(&form_data, APPEND_INAME)),
                    dir: form_data
                        .get(BASE_DIR_INAME)
                        .as_string()
                        .and_then(|e| e.parse::<u8>().ok())
                        .and_then(BaseDirectory::from_repr),
                });
                let path = form_data
                    .get(FILE_PATH_INAME)
                    .as_string()
                    .filter(|c| !c.is_empty())
                    .ok_or(anyhow::format_err!("Invalid/missing `path` input"))?;
                match serde_wasm_bindgen::from_value::<ReadActionType>(
                    form_data.get(ACTION_TYPE_SELECT),
                )
                .map_err(|err| anyhow::format_err!("{err}"))?
                {
                    ReadActionType::Binary => read_binary_file(&path, options)
                        .await
                        .map(ReadActionReturn::Binary)
                        .map_err(|err| anyhow::format_err!("{err}")),
                    ReadActionType::Text => read_text_file(&path, options)
                        .await
                        .map(ReadActionReturn::Text)
                        .map_err(|err| anyhow::format_err!("{err}")),
                }
            }
        });
    let action_value = action.value();
    let action_pending = action.pending();
    view! {
        <div>
            {move || {
                match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                    Some(Err(err)) => {
                        view! { <p style:color="red">{format!("{err}")}</p> }.into_any()
                    }
                    _ => ().into_any(),
                }
            }}
            <form on:submit=move |ev| {
                ev.prevent_default();
                action.dispatch_local(extract_form_data(&ev));
            }>
                <input type="text" name=FILE_PATH_INAME placeholder="File path" />
                <select name=ACTION_TYPE_SELECT>
                    {ReadActionType::all_variants()
                        .iter()
                        .map(|variant| {
                            view! {
                                <option value=serde_wasm_bindgen::to_value(variant)
                                    .ok()
                                    .and_then(|e| e.as_string())>{format!("{:?}", variant)}</option>
                            }
                        })
                        .collect_view()}
                </select>
                <FsOptionInputs
                    append_input_name=APPEND_INAME
                    base_dir_select_name=BASE_DIR_INAME
                />
                <button type="submit">"Read..."</button>
            </form>
            {move || {
                match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                    Some(Ok(ReadActionReturn::Binary(bin))) => {
                        view! {
                            <code>
                                {bin
                                    .iter()
                                    .enumerate()
                                    .map(|(i, e)| {
                                        view! {
                                            {*e}
                                            ";"
                                            {if i % 25 == 0 && i != 1 {
                                                view! { <br /> }.into_any()
                                            } else {
                                                ().into_any()
                                            }}
                                        }
                                    })
                                    .collect_view()}
                            </code>
                        }
                            .into_any()
                    }
                    Some(Ok(ReadActionReturn::Text(text))) => {
                        view! {
                            <code>
                                {text
                                    .split('\n')
                                    .map(|content| {
                                        view! {
                                            {String::from(content)}
                                            <br />
                                        }
                                    })
                                    .collect_view()}
                            </code>
                        }
                            .into_any()
                    }
                    _ => ().into_any(),
                }
            }}
        </div>
    }
}
