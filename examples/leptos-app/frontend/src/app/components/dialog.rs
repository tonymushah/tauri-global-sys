stylance::import_crate_style!(pub(self) styles, "src/app/components/dialog.module.scss");
mod ask;
mod confirm;

use ask::Ask;
use confirm::Confirm;
use leptos::prelude::*;

#[component]
pub fn Dialog() -> impl IntoView {
    view! {
        <div class=styles::top_section>
            <section>
                <h3>"Ask"</h3>
                <Ask />
            </section>
            <section>
                <h3>"Confirm"</h3>
                <Confirm />
            </section>
        </div>
    }
}
