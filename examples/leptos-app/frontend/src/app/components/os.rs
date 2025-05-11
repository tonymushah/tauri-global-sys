use std::{error::Error, fmt::Debug};

use leptos::prelude::*;
use tauri_global_sys::os::{arch, locale, platform, tempdir, type_, version};

#[component]
fn LocalResourceView<T: Debug + 'static, E: Error + 'static>(
    res: LocalResource<Result<T, E>>,
) -> impl IntoView {
    move || match res.read().as_ref() {
        None => view! { <i>"Loading"</i> }.into_any(),
        Some(Ok(d)) => view! { <span>{format!("{d:?}")}</span> }.into_any(),
        Some(Err(err)) => {
            view! { <strong style:color="red">{format!("{err}")}</strong> }.into_any()
        }
    }
}

#[component]
pub fn Os() -> impl IntoView {
    let arch = LocalResource::new(arch);
    let locale = LocalResource::new(locale);
    let platform = LocalResource::new(platform);
    let tempdir = LocalResource::new(tempdir);
    let type_ = LocalResource::new(type_);
    let version = LocalResource::new(version);
    view! {
        <p on:click=move |_| arch.refetch()>"Arch: " <LocalResourceView res=arch /></p>
        <p on:click=move |_| locale.refetch()>"Locale: " <LocalResourceView res=locale /></p>
        <p on:click=move |_| platform.refetch()>"Platform: " <LocalResourceView res=platform /></p>
        <p on:click=move |_| tempdir.refetch()>"Tempdir: " <LocalResourceView res=tempdir /></p>
        <p on:click=move |_| type_.refetch()>"Locale: " <LocalResourceView res=type_ /></p>
        <p on:click=move |_| version.refetch()>"Version: " <LocalResourceView res=version /></p>
    }
}
