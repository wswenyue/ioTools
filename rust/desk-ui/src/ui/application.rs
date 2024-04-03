use std::path::PathBuf;

use tauri::{Config};

pub fn app_root() -> PathBuf {
    tauri::api::path::home_dir().unwrap().join(".iotools")
}

pub fn get_tauri_conf() -> Option<Config> {
    let config_file = include_str!("../../tauri.conf.json");
    let config: Config = serde_json::from_str(config_file).expect("failed to parse tauri.conf.json");
    Some(config)
}
