use leptos::prelude::*;
use tauri_global_sys::app::{get_name, get_version};

#[component]
fn AppName() -> impl IntoView {
    let app_name_res = LocalResource::new(|| async { get_name().await });
    let app_name = move || {
        app_name_res.read().as_ref().map(|res| match res {
            Ok(app_name) => view! { <span>"App name: " {app_name.clone()}</span> }.into_any(),
            Err(err) => view! { <span style:color="red">{format!("{err}")}</span> }.into_any(),
        })
    };
    view! {
        <Show
            when=move || { app_name_res.read().is_some() }
            fallback=|| {
                view! { <p>"Loading app name..."</p> }
            }
        >
            <p on:click=move |_| {
                app_name_res.refetch();
            }>{app_name}</p>
        </Show>
    }
}

#[component]
fn AppVersion() -> impl IntoView {
    let app_ver_res = LocalResource::new(|| async { get_version().await });
    let app_ver = move || {
        app_ver_res.read().as_ref().map(|res| match res {
            Ok(ver) => view! { <span>"App Version: " {format!("{ver}")}</span> }.into_any(),
            Err(err) => view! { <span style:color="red">{format!("{err}")}</span> }.into_any(),
        })
    };
    view! {
        <Show
            when=move || { app_ver_res.read().is_some() }
            fallback=|| {
                view! { <p>"Loading app version..."</p> }
            }
        >
            <p on:click=move |_| {
                app_ver_res.refetch();
            }>{app_ver}</p>
        </Show>
    }
}

#[component]
fn TauriVersion() -> impl IntoView {
    let tauri_ver_res = LocalResource::new(|| async { get_version().await });
    let tauri_ver = move || {
        tauri_ver_res.read().as_ref().map(|res| match res {
            Ok(ver) => view! { <span>"Tauri Version: " {format!("{ver}")}</span> }.into_any(),
            Err(err) => view! { <span style:color="red">{format!("{err}")}</span> }.into_any(),
        })
    };
    view! {
        <Show
            when=move || { tauri_ver_res.read().is_some() }
            fallback=|| {
                view! { <p>"Loading Tauri version..."</p> }
            }
        >
            <p on:click=move |_| {
                tauri_ver_res.refetch();
            }>{tauri_ver}</p>
        </Show>
    }
}

#[component]
pub fn AppModule() -> impl IntoView {
    view! {
        <div>
            <AppName />
            <AppVersion />
            <TauriVersion />
        </div>
    }
}
