pub mod base_directory;
pub mod copy_file;
pub mod create_dir;
pub mod fs_dir_option;
pub mod fs_option;

use copy_file::CopyFile;
use create_dir::CreateDir;
use leptos::prelude::*;

#[component]
pub fn Fs() -> impl IntoView {
    view! {
        <div>
            <section>
                <h4>"Copy File"</h4>
                <CopyFile />
            </section>
            <section>
                <h4>"Create dir"</h4>
                <CreateDir />
            </section>
        </div>
    }
}
