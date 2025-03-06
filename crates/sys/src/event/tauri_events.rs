use super::raw::tauri_event;

pub fn check_update() -> String {
    tauri_event::CHECK_UPDATE.with(Clone::clone)
}

pub fn download_progress() -> String {
    tauri_event::DOWNLOAD_PROGRESS.with(Clone::clone)
}

pub fn install_update() -> String {
    tauri_event::INSTALL_UPDATE.with(Clone::clone)
}

pub fn menu() -> String {
    tauri_event::MENU.with(Clone::clone)
}

pub fn status_update() -> String {
    tauri_event::STATUS_UPDATE.with(Clone::clone)
}

pub fn update_available() -> String {
    tauri_event::UPDATE_AVAILABLE.with(Clone::clone)
}

pub fn window_blur() -> String {
    tauri_event::WINDOW_BLUR.with(Clone::clone)
}

pub fn window_close_requested() -> String {
    tauri_event::WINDOW_CLOSE_REQUESTED.with(Clone::clone)
}

pub fn window_created() -> String {
    tauri_event::WINDOW_CREATED.with(Clone::clone)
}

pub fn window_destroyed() -> String {
    tauri_event::WINDOW_DESTROYED.with(Clone::clone)
}

pub fn window_file_drop() -> String {
    tauri_event::WINDOW_FILE_DROP.with(Clone::clone)
}

pub fn window_file_drop_cancelled() -> String {
    tauri_event::WINDOW_FILE_DROP_CANCELLED.with(Clone::clone)
}

pub fn window_file_drop_hover() -> String {
    tauri_event::WINDOW_FILE_DROP_HOVER.with(Clone::clone)
}

pub fn window_focus() -> String {
    tauri_event::WINDOW_FOCUS.with(Clone::clone)
}

pub fn window_moved() -> String {
    tauri_event::WINDOW_MOVED.with(Clone::clone)
}

pub fn window_resized() -> String {
    tauri_event::WINDOW_RESIZED.with(Clone::clone)
}

pub fn window_scale_factor_changed() -> String {
    tauri_event::WINDOW_SCALE_FACTOR_CHANGED.with(Clone::clone)
}

pub fn window_theme_changed() -> String {
    tauri_event::WINDOW_THEME_CHANGED.with(Clone::clone)
}
