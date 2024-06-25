use encoding_rs::Encoding;
use ini::Ini;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::utils::get_temp_path;

pub struct Config {
    pub config: HashMap<String, String>,
}

/// Get HPM config ：接受.hpm或.config或含HPM.config的目录
pub fn get_hpm_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(if path.is_dir() {
        path.join("HPM.config")
    } else {
        let file_extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or("Failed to get file extension")?
            .to_lowercase();

        match file_extension.as_str() {
            "config" => path.to_owned(),
            "hpm" => {
                let output = std::process::Command::new(
                    get_temp_path()
                        .join("7z")
                        .join("7z.exe")
                        .to_str()
                        .ok_or("Failed to get 7z path")?,
                )
                .args(&[
                    "x",
                    path.to_str().unwrap(),
                    "HPM.config",
                    &format!(
                        "-o{}",
                        get_temp_path().to_str().ok_or("Failed to get temp path")?
                    ),
                ])
                .output()?;

                if !output.status.success() {
                    return Err("Failed to extract HPM.config".into());
                }

                get_temp_path().join("HPM.config")
            }
            _ => return Err("Invalid config file extension".into()),
        }
    })?;

    let mut buffer = Vec::new();
    BufReader::new(file).read_to_end(&mut buffer)?;

    // 文件是GBK编码
    let encoding = Encoding::for_label(b"GBK").ok_or("Unsupported encoding GBK")?;
    let (decoded, _, _) = encoding.decode(&buffer);

    let ini = Ini::load_from_str(&decoded)?;
    let mut config = Config {
        config: HashMap::new(),
    };

    ini.iter().for_each(|(section, values)| {
        values.iter().for_each(|(key, value)| {
            config.config.insert(
                format!("{}.{}", section.unwrap_or_default(), key),
                value.to_string(),
            );
        });
    });

    Ok(config)
}
