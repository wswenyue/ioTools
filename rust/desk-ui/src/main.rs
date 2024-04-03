use chrono::{DateTime, Utc};
use tauri::{Context, Manager};
use tauri::utils::assets::EmbeddedAssets;
use tauri::utils::config::BeforeDevCommand;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};
use tauri_plugin_log::LogTarget;

use crate::ui::{application, menu};

mod ui;
mod  tools;


#[tauri::command]
fn greet(name: &str) -> String {
   let current_utc: DateTime<Utc> = Utc::now();
   format!("Rust Hello, {}!-->{}", name,current_utc)
}

fn main() {
    let mut log = tauri_plugin_log::Builder::default()
        .targets([
            // LogTarget::LogDir,
            // LOG PATH: ~/.chatgpt/ChatGPT.log
            LogTarget::Folder(application::app_root()),
            LogTarget::Stdout,
            LogTarget::Webview,
        ])
        .level(log::LevelFilter::Info);

    if cfg!(debug_assertions) {
        log = log.with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::Blue,
            info: Color::BrightGreen,
            trace: Color::Cyan,
        });
    }

    tauri::Builder::default()
        .plugin(log.build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                None,
            )
        )
        .invoke_handler(tauri::generate_handler![
            greet
            // get_product_info,
            // get_ticket_list,
            // get_ticket_detail,
            // create_order,
            // get_user_list,
            // // version::get_repo_version,
            // utils::export_sql_to_txt,
        ])
        // .setup(application::init)
        .menu(menu::init())
        .run(tauri::generate_context!())
        .expect("error while running application");
}