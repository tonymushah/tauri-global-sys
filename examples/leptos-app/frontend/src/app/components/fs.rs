pub mod base_directory;
pub mod copy_file;
pub mod create_dir;
pub mod exists;
pub mod fs_dir_option;
pub mod fs_option;
pub mod read_dir;
pub mod read_file;
pub mod remove_smth;
pub mod rename_file;
pub mod write_file;

use copy_file::CopyFile;
use create_dir::CreateDir;
use exists::Exists;
use leptos::prelude::*;
use read_dir::ReadDir;
use read_file::ReadFile;
use remove_smth::RemoveSmth;
use rename_file::RenameFile;
use write_file::WriteFile;

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
            <section>
                <h4>"Read directory"</h4>
                <ReadDir />
            </section>
            <section>
                <h4>"Remove file/directory"</h4>
                <RemoveSmth />
            </section>
            <section>
                <h4>"Rename/Move file"</h4>
                <RenameFile />
            </section>
            <section>
                <h4>"Write file"</h4>
                <WriteFile />
            </section>
        </div>
    }
}
