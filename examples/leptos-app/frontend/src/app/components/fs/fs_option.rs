use leptos::prelude::*;

use crate::app::components::fs::base_directory::BaseDirectorySelect;

#[component]
pub fn FsOptionInputs(
    #[prop(into)] append_input_name: String,
    #[prop(into, optional)] base_dir_select_name: Option<String>,
) -> impl IntoView {
    view! {
        <div>
            <label for="">
                <input type="checkbox" name=append_input_name placeholder="Append" />
                " Append"
            </label>
            <br />
            <BaseDirectorySelect
                name=base_dir_select_name.unwrap_or("dir".into())
                placeholder="Base directory"
            />
        </div>
    }
}
