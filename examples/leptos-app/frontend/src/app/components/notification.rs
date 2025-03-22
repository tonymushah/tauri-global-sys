use futures::TryFutureExt;
use leptos::prelude::*;
use tauri_global_sys::notification::{
    is_permission_granted, request_permission, send_notification,
    Options as SendNotificationOptions, Permission,
};

#[component]
pub fn Notification() -> impl IntoView {
    let action = Action::new_local(|_: &()| async move {
        let mut permission_granted = is_permission_granted()
            .map_err(|err| anyhow::format_err!("{err}"))
            .await?;
        if !permission_granted {
            let permission = request_permission()
                .map_err(|err| anyhow::format_err!("{err}"))
                .await?;
            permission_granted = permission == Permission::Granted;
        }
        if permission_granted {
            send_notification("Tauri is awesome!".into())
                .map_err(|err| anyhow::format_err!("{err}"))?;
            send_notification(SendNotificationOptions {
                title: "TAURI".into(),
                body: Some("Tauri is awesome!".into()),
                sound: None,
                icon: None,
            })
            .map_err(|err| anyhow::format_err!("{err}"))?;
        }
        anyhow::Ok(())
    });
    let action_pending = action.pending();
    let action_value = action.value_local().read_only();
    view! {
        {move || {
            match action_value.read().as_ref().filter(|_| action_pending.read() == false) {
                Some(Err(err)) => view! { <p style:color="red">{format!("{err}")}</p> }.into_any(),
                _ => ().into_any(),
            }
        }}
        <button on:click=move |_| {
            action.dispatch_local(());
        }>"Notification"</button>
    }
}
