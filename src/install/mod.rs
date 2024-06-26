use load::{load_driver_pack, run_pecmd_script};

use crate::utils::{compression::sevenz::decompress_file_7z, is_user_an_admin};

mod load;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Output,
};
pub fn hpm_install(
    hpm_path: &PathBuf,
    install_path: &PathBuf,
    hpm_config: HashMap<String, String>,
) {
    let input_path = &Path::new(hpm_path).to_path_buf();

    println!(
        "Installing: {}",
        hpm_config.get("HPM_config.mod_name").unwrap()
    );

    match decompress_file_7z(
        input_path,
        install_path,
        |_progress, _total| {
            //format!("Decompression progress: {} out of {}", progress, total);
        },None
    ) {
        Err(e) => eprintln!("Decompression failed: {}", e),
        _ => {}
    }

    hpm_executor(install_path, hpm_config);
}

pub fn hpm_executor(install_path: &PathBuf, hpm_config: HashMap<String, String>) -> Output {
    let binding = String::from("Software");
    let pack_mode = hpm_config.get("Packaging_mode").unwrap_or(&binding);

    match pack_mode.as_str() {
        "Driver" => {
            if !is_user_an_admin().unwrap_or(true) {
                panic!("Please run as administrator to install driver.");
            };
            println!("Load driver pack...");
            load_driver_pack(&install_path.join("Driver.7z"))
        }
        _ => {
            println!("Run installation script...");
            run_pecmd_script(&install_path.join("HPM_Next.WCE"))
        }
    }
}
