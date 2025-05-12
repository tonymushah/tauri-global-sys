use leptos::prelude::*;
use tauri_global_sys::{
    dialog::{open, OpenDialogOptions},
    fs::{remove_dir, remove_file, FsDirOptions},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RemoveType {
    File,
    Directory,
    DirectoryRecursive,
}

#[component]
pub fn RemoveSmth() -> impl IntoView {
    let action = Action::<_, anyhow::Result<_>>::new_local(|type_: &RemoveType| {
        let type_ = *type_;
        async move {
            let to_remove = open(
                OpenDialogOptions {
                    directory: (type_ == RemoveType::Directory).into(),
                    title: match type_ {
                        RemoveType::File => String::from("Select file to remove").into(),
                        RemoveType::Directory => String::from("Select directory to remove").into(),
                        RemoveType::DirectoryRecursive => {
                            String::from("Select directory to remove recursivly").into()
                        }
                    },
                    ..Default::default()
                }
                .into(),
            )
            .await
            .map_err(|e| anyhow::format_err!("{e}"))?
            .and_then(|r| r.single())
            .ok_or(anyhow::format_err!("Please select a directory"))?;
            match type_ {
                RemoveType::File => {
                    remove_file(&to_remove, None)
                        .await
                        .map_err(|err| anyhow::format_err!("{err}"))?;
                }
                RemoveType::Directory | RemoveType::DirectoryRecursive => {
                    remove_dir(
                        &to_remove,
                        FsDirOptions {
                            recursive: (type_ == RemoveType::DirectoryRecursive).into(),
                            ..Default::default()
                        }
                        .into(),
                    )
                    .await
                    .map_err(|err| anyhow::format_err!("{err}"))?;
                }
            }
            Ok((to_remove, type_))
        }
    });
    let action_value = action.value();
    let action_pending = action.pending();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                Some(Ok((path, type_))) => {
                    view! {
                        <p style:color="green">
                            {format!(
                                "Removed {path} {}",
                                match type_ {
                                    RemoveType::File => "file",
                                    RemoveType::Directory => "directory",
                                    RemoveType::DirectoryRecursive => "directory recursively",
                                },
                            )}
                        </p>
                    }
                        .into_any()
                }
                _ => ().into_any(),
            }
        }}
        <button
            disabled=action_pending
            on:click=move |_| {
                action.dispatch_local(RemoveType::File);
            }
        >
            "Remove file"
        </button>
        <button
            disabled=action_pending
            on:click=move |_| {
                action.dispatch_local(RemoveType::Directory);
            }
        >
            "Remove directory"
        </button>
        <button
            disabled=action_pending
            on:click=move |_| {
                action.dispatch_local(RemoveType::DirectoryRecursive);
            }
        >
            "Remove directory recursively"
        </button>
    }
}
