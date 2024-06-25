use std::fs::{self, File};
use std::io::Write;
use tempfile::{tempdir, TempDir};
use std::convert::AsRef;

use crate::{Asset, TEMP_DIR_PATH};

pub fn extract_assets_to_temp() -> std::io::Result<TempDir> {
    let temp_dir = tempdir()?;
    let temp_dir_path = temp_dir.path().to_path_buf();

    for file in Asset::iter() {
        if let Some(content) = Asset::get(&file) {
            let temp_file_path = temp_dir_path.join(file.as_ref());

            if let Some(parent) = temp_file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut temp_file = File::create(&temp_file_path)?;
            temp_file.write_all(&content.data.as_ref())?;
        }
    }

    let mut temp_path_guard = TEMP_DIR_PATH.lock().unwrap();
    *temp_path_guard = Some(temp_dir_path);

    Ok(temp_dir)
}