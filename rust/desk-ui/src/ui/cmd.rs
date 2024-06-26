use std::{fs, path::PathBuf};
use tauri::{api, command, AppHandle, Manager, WindowBuilder, WindowUrl};
use crate::ui::application::app_root;
use crate::tools::comm_utils;
use crate::tools::comm_utils::create_file;

#[command]
pub fn drag_window(app: AppHandle) {
    app.get_window("core").unwrap().start_dragging().unwrap();
}

#[command]
pub fn fullscreen(app: AppHandle) {
    let win = app.get_window("core").unwrap();
    if win.is_fullscreen().unwrap() {
        win.set_fullscreen(false).unwrap();
    } else {
        win.set_fullscreen(true).unwrap();
    }
}

// #[command]
// pub fn download(ui: AppHandle, name: String, blob: Vec<u8>) {
//   let win = ui.app_handle().get_window("core");
//   let path = utils::app_root().join(PathBuf::from(name));
//   utils::create_file(&path).unwrap();
//   fs::write(&path, blob).unwrap();
//   tauri::api::dialog::message(
//     win.as_ref(),
//     "Save File",
//     format!("PATH: {}", path.display()),
//   );
// }

#[command]
pub fn save_file(_app: AppHandle, name: String, content: String) {
    // let win = ui.app_handle().get_window("core");
    let path = app_root().join(PathBuf::from(name));
    create_file(&path).unwrap();
    fs::write(&path, content).unwrap();
    comm_utils::open_file(path);
    // tauri::api::dialog::message(
    //   win.as_ref(),
    //   "Save File",
    //   format!("PATH: {}", path.display()),
    // );
}

#[command]
pub fn open_link(app: AppHandle, url: String) {
    api::shell::open(&app.shell_scope(), url, None).unwrap();
}

#[command]
pub fn run_check_update(app: AppHandle, silent: bool, has_msg: Option<bool>) {
    // utils::run_check_update(ui, silent, has_msg);
    //TODO
}

#[command]
pub fn open_file(path: PathBuf) {
    comm_utils::open_file(path);
}


#[command]
pub fn download_file(name: String, blob: Vec<u8>) {
    let file = api::path::download_dir().unwrap().join(name);
    fs::write(&file, blob).unwrap();
    comm_utils::open_file(file);
}

// #[command]
// pub async fn get_data(app: AppHandle, url: String, is_msg: Option<bool>) -> Option<String> {
//     let is_msg = is_msg.unwrap_or(false);
//     let res = if is_msg {
//         utils::get_data(&url, Some(&app)).await
//     } else {
//         utils::get_data(&url, None).await
//     };
//     res.unwrap_or_else(|err| {
//         error!("chatgpt_client_http: {}", err);
//         None
//     })
// }

// #[tauri::command]
// pub async fn fetch_image(url: String) -> Vec<u8> {
//     let response = reqwest::get(url).await.unwrap();
//     let bytes = response.bytes().await.unwrap();
//     bytes.to_vec()
// }


#[command]
pub fn window_reload(app: AppHandle, label: &str) {
    app.app_handle()
        .get_window(label)
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
}