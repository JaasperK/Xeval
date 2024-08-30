use std::path::PathBuf;
use rfd::FileDialog;

pub fn get_path() -> Option<PathBuf> {
    if let Some(path) = FileDialog::new().pick_file() {
        Some(path)
    } else {
        None
    }
}