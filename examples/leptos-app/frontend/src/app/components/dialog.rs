stylance::import_crate_style!(pub(self) styles, "src/app/components/dialog.module.scss");
use leptos::prelude::*;

mod ask;
mod confirm;
mod message;
mod open;

use ask::Ask;
use confirm::Confirm;
use message::Message;
use open::Open;

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
            <section>
                <h3>"Message"</h3>
                <Message />
            </section>
            <section>
                <h3>"Open"</h3>
                <Open />
            </section>
        </div>
    }
}
