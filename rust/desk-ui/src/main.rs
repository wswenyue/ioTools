use chrono::{DateTime, Utc};
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
   let current_utc: DateTime<Utc> = Utc::now();
   format!("Rust Hello, {}!-->{}", name,current_utc)
}
fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        // .plugin(tauri_plugin_sql::Builder::default().build())
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
        .run(tauri::generate_context!("./tauri.conf.json"))
        .expect("error while running tauri application");
}