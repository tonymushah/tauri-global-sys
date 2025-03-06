stylance::import_crate_style!(styles, "src/app/components/test_event.module.css");

use futures::{stream::abortable, StreamExt};
use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use tauri_global_sys::event::{emit, listen};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::app::hooks::use_aborts::{use_stream_aborts, UseAbortReturns};

const EVENT_NAME: &str = "test-listen";

#[component]
pub fn TestEventReceive() -> impl IntoView {
    let (event_payload, set_event_payload) = signal(String::new());
    let UseAbortReturns { push_handle, .. } = use_stream_aborts(true);

    spawn_local(async move {
        let (mut stream, handle) = abortable(listen::<String>(EVENT_NAME).await.unwrap());
        push_handle(handle);
        while let Some(event) = stream.next().await {
            set_event_payload.set(event.payload);
        }
    });

    view! { <p>{move || { event_payload.get() }}</p> }
}

#[component]
pub fn TestEventSend() -> impl IntoView {
    let on_form_submit = move |event: SubmitEvent| {
        spawn_local(async move {
            event.prevent_default();
            let form: HtmlFormElement = event
                .current_target()
                .expect("Need a current target")
                .unchecked_into();
            let form_data = FormData::new_with_form(&form).unwrap();
            emit(
                EVENT_NAME,
                &form_data
                    .get("content")
                    .as_string()
                    .expect("Invalid data: expected string"),
            )
            .await
            .unwrap();
        });
    };
    view! {
        <form class="row" on:submit=on_form_submit>
            <input name="content" type="text" placeholder="Message..." />
            <button type="submit" class=styles::button>
                "Send"
            </button>
        </form>
    }
}

#[component]
pub fn TestEvent() -> impl IntoView {
    view! {
        <div class=styles::test_event>
            <TestEventSend />
            <TestEventReceive />
        </div>
    }
}
