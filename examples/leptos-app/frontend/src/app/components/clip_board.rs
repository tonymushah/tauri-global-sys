use leptos::{html::Input, prelude::*};
use tauri_global_sys::clipboard::{read_text, write_text};

#[component]
fn ReadText() -> impl IntoView {
    let clip_board_res = LocalResource::new(|| async { read_text().await });
    let clip_board_data = move || {
        clip_board_res.read().as_deref().map(|res| match res {
            Ok(maybe_content) => {
                let maybe_content = maybe_content.clone();
                view! {
                    <span>
                        "Clipboard Content: "
                        <Suspense fallback=|| {
                            view! { <i>"Nothing in clipboard"</i> }
                        }>{maybe_content}</Suspense>
                    </span>
                }
                .into_any()
            }
            Err(err) => view! { <span style:color="red">{format!("{err}")}</span> }.into_any(),
        })
    };
    view! {
        <Show
            when=move || { clip_board_res.read().is_some() }
            fallback=|| {
                view! { <p>"Loading clipboard content..."</p> }
            }
        >
            <p on:click=move |_| {
                clip_board_res.refetch();
            }>{clip_board_data}</p>
        </Show>
    }
}

#[component]
fn WriteText() -> impl IntoView {
    let write_text_action = Action::new_local(|text: &String| {
        let text = text.clone();
        async move { write_text(&text).await.map_err(|e| e.to_string()) }
    });
    let write_text_action_data = write_text_action.value();
    let write_input_ref = NodeRef::<Input>::new();
    view! {
        {move || {
            match write_text_action_data.read().as_ref() {
                Some(Err(err)) => view! { <p style:color="red">{err.clone()}</p> }.into_any(),
                _ => None::<()>.into_any(),
            }
        }}
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = write_input_ref.get().expect("input to exist");
            write_text_action.dispatch_local(input.value());
        }>
            <label>
                "Please write something..." <input type="text" node_ref=write_input_ref />
            </label>
            <button type="submit">"Write"</button>
        </form>
    }
}

#[component]
pub fn ClipBoard() -> impl IntoView {
    view! {
        <div>
            <ReadText />
            <WriteText />
        </div>
    }
}
