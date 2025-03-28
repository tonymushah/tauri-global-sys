use std::fmt::Debug;

use leptos::prelude::*;

#[component]
pub fn FormatDebug<'d, D: Debug>(dbg: &'d D) -> impl IntoView {
    let data = format!("{:#?}", dbg);
    data.lines()
        .map(|line| {
            view! { <span>{String::from(line)}</span> }
        })
        .collect_view()
}
