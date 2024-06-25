use std::{path::PathBuf, process};

use crate::utils::get_temp_path;

/// 运行pecmd脚本，接受脚本路径(如.wce)
 pub fn run_pecmd_script(script_path: &PathBuf)-> process::Output {
    let output =
        process::Command::new(get_temp_path().join("pecmd.exe").to_str().unwrap())
            .args(&["LOAD", script_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute pecmd command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        output
 }

///加载驱动包,接受驱动包路径(如.7z)
 pub fn load_driver_pack(driver_pack_path: &PathBuf)-> process::Output {
    let output =
        process::Command::new(get_temp_path().join("DrvIndex.exe").to_str().unwrap())
            .args(&["-b", driver_pack_path.to_str().unwrap(),"-hide"])
            .output()
            .expect("Failed to execute DrvIndex command");
        print!("{}", String::from_utf8_lossy(&output.stdout));
        output
    }