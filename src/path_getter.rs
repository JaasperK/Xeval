use std::path::PathBuf;
use rfd::FileDialog;

pub fn get_path() -> Option<PathBuf> {
    FileDialog::new().pick_file()
}
