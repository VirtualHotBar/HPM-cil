use config::get_hpm_config;
use dirs::home_dir;
use rust_embed::RustEmbed;

use lazy_static::lazy_static;
use std::path::PathBuf;

use std::sync::Mutex;

mod utils;

mod install;
use install::{hpm_executor, hpm_install};
use utils::res::extract_assets_to_temp;

mod config;

use clap::{Arg, Command};

//嵌入资源
#[derive(RustEmbed)]
#[folder = "res"]
pub struct Asset;

//临时目录
lazy_static! {
    pub static ref TEMP_DIR_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
}

fn main() {
    let matches = Command::new("HPMMGR - CLI")
        .version("0.1")
        .author("VirtualHotBar <blog.hotpe.top>")
        .about("HPM(HotPE Module) Manager")
        .arg_required_else_help(true)
        .arg(
            Arg::new("HPMPath")
                .help("Sets the input file to use")
                .index(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .value_name("PATH")
                .help("Install HPM package")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("quickLoad")
                .short('q')
                .long("quickload")
                .value_name("QuickPATH")
                .help("Quickly load HPM package")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("installPath")
                .long("installpath")
                .value_name("installPath")
                .help("Set install path")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    // 提取资源
    let _res_dir = extract_assets_to_temp();

    //默认: ~/HPM
    let install_path_str = matches
        .get_one::<String>("installPath")
        .map(String::from)
        .unwrap_or_else(|| {
            home_dir()
                .unwrap()
                .join("HPM")
                .to_str()
                .unwrap()
                .to_string()
        });

    // 将字符串转换为`PathBuf`
    let install_path = &PathBuf::from(install_path_str);

    let hpm_path = matches
        .get_one::<String>("install")
        .or_else(|| matches.get_one::<String>("HPMPath"));
    // 安装
    if let Some(hpm_path) = hpm_path {
        let hpm_path = &PathBuf::from(hpm_path);
        if hpm_path.is_file() {
            let hpm_config = get_hpm_config(hpm_path).unwrap().config;

            hpm_install(
                hpm_path,
                &install_path.join(hpm_config.get("HPM_config.mod_name").unwrap()),
                hpm_config,
            );
        } else {
            panic!("Invalid HPM file path,cannot be a directory")
        };

        println!("Done")
    }

    //快速加载
    if let Some(quick_path) = matches.get_one::<String>("quickLoad") {
        let quick_path = &PathBuf::from(quick_path);
        println!("QuickLoad: {}", quick_path.to_str().unwrap());

        if quick_path.is_dir() {
            for entry in quick_path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() && path.join("HPM.config").is_file() {
                    let hpm_config = get_hpm_config(&path).unwrap().config;
                    hpm_executor(&path, hpm_config);
                }
            }
        } else {
            panic!("Invalid QuickLoad path,cannot be a file")
        };
    }
}
