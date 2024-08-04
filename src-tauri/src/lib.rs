pub mod database;
pub mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle();
                
            let app_state:State<AppState> = handle.state();
            let db = database::initialize_database(&app).expect("Database intiialization failed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
