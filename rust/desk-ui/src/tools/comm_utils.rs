use log::info;
use std::{collections::HashMap, fs, io, path::{Path, PathBuf}, process::Command};

pub fn exists(path: &Path) -> bool {
    Path::new(path).exists()
}

pub fn create_file<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let filename = filename.as_ref();
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::File::create(filename)?;
    Ok(())
}

pub fn convert_path(path_str: &str) -> String {
    if cfg!(target_os = "windows") {
        path_str.replace('/', "\\")
    } else {
        String::from(path_str)
    }
}

pub fn open_file(path: PathBuf) {
    let pathname = convert_path(path.to_str().unwrap());
    info!("open_file: {}", pathname);
    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg("-R")
        .arg(pathname)
        .spawn()
        .unwrap();

    #[cfg(target_os = "windows")]
    Command::new("explorer.exe")
        .arg("/select,")
        .arg(pathname)
        .spawn()
        .unwrap();

    // https://askubuntu.com/a/31071
    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(pathname).spawn().unwrap();
}