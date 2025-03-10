use leptos::prelude::*;

use crate::app::components::fs::base_directory::BaseDirectorySelect;

#[component]
pub fn FsDirOptionInputs(
    #[prop(into)] recursive_input_name: String,
    #[prop(into, optional)] base_dir_select_name: Option<String>,
) -> impl IntoView {
    view! {
        <div>
            <label for=recursive_input_name
                .clone()>
                "Recursive" <input type="checkbox" name=recursive_input_name.clone() />
            </label>

            <BaseDirectorySelect
                name=base_dir_select_name.unwrap_or("dir".into())
                placeholder="Base directory"
            />
        </div>
    }
}
