stylance::import_crate_style!(pub(self) styles, "src/app/components/dialog.module.scss");
mod ask;

use ask::Ask;
use leptos::prelude::*;

#[component]
pub fn Dialog() -> impl IntoView {
    view! {
        <div class=styles::top_section>
            <section>
                <h3>"Ask"</h3>
                <Ask />
            </section>
        </div>
    }
}
