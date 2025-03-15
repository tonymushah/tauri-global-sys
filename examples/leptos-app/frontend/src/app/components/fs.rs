pub mod base_directory;
pub mod copy_file;
pub mod create_dir;
pub mod exists;
pub mod fs_dir_option;
pub mod fs_option;
pub mod read_file;

use copy_file::CopyFile;
use create_dir::CreateDir;
use exists::Exists;
use leptos::prelude::*;
use read_file::ReadFile;

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
            <section>
                <h4>"Exists"</h4>
                <Exists />
            </section>
            <section>
                <h4>"Read file"</h4>
                <ReadFile />
            </section>
        </div>
    }
}
