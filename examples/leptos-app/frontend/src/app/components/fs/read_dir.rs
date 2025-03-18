stylance::import_crate_style!(styles, "src/app/components/fs/read_dir.module.scss");

use leptos::prelude::*;
use tauri_global_sys::{
    dialog::{open, OpenDialogOptions, OpenReturnType},
    fs::{read_dir, FileEntry, FsDirOptions},
};

#[component]
fn FileEntryComponnent(entry: FileEntry) -> impl IntoView {
    let (show, set_show) = signal(false);
    let childrens = entry.children.unwrap_or_default();

    view! {
        <div class=styles::file_entry>
            <p
                class=format!(
                    "{} {}",
                    styles::p,
                    if childrens.is_empty() { styles::is_file } else { styles::is_directory },
                )
                on:click=move |_| {
                    set_show.update(|inner| *inner = !*inner);
                }
            >
                {if let Some(name) = entry.name {
                    format!("{name} ({})", entry.path)
                } else {
                    format!("({})", entry.path)
                }}
            </p>
            {move || {
                if !childrens.is_empty() && *show.read() {
                    view! {
                        <div class=styles::file_entry_childrens>
                            {childrens
                                .iter()
                                .map(|item| {
                                    view! { <FileEntryComponnent entry=item.clone() /> }
                                })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn ReadDir() -> impl IntoView {
    let action = Action::<_, anyhow::Result<Vec<FileEntry>>, _>::new_local(|_: &()| async move {
        let dir = open(Some(OpenDialogOptions {
            directory: Some(true),
            title: Some(String::from("Select a directory")),
            ..Default::default()
        }))
        .await
        .map_err(|err| anyhow::format_err!("{err}"))?
        .and_then(OpenReturnType::single)
        .ok_or(anyhow::format_err!("Please select a directory"))?;
        read_dir(
            &dir,
            Some(FsDirOptions {
                recursive: true.into(),
                ..Default::default()
            }),
        )
        .await
        .map_err(|err| anyhow::format_err!("{err}"))
    });
    let action_pending = action.pending();
    let action_value = action.value_local();
    view! {
        {move || {
            match action_value.read().as_ref() {
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                _ => ().into_any(),
            }
        }}
        <button
            class=styles::read_dir_button
            on:click=move |_| {
                action.dispatch_local(());
            }
            disabled=action_pending
        >
            "Read a directory..."
        </button>
        <div class=styles::read_dir>
            <For
                each=move || {
                    action_value
                        .read()
                        .as_ref()
                        .and_then(|v| v.as_ref().ok())
                        .cloned()
                        .unwrap_or_default()
                }
                key=|item| item.path.clone()
                let:item
            >
                <FileEntryComponnent entry=item />
            </For>
        </div>
    }
}
