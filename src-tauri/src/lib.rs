pub mod database;
pub mod state;
pub mod dbstructs;

use tauri::{Manager, State, AppHandle};
use state::{ServiceAccess, AppState};
use dbstructs::{NewSet, Set};

#[tauri::command]
fn add_set(name: &str, description: &str, app_handle: AppHandle) -> String {
    let result = app_handle.conn(|conn| database::add_set(NewSet{name, description}, conn));

    match result {
        Ok(_) => String::new(),
        Err(msg) => msg.to_string()
    }

    /*let items = app_handle.conn(|conn| database::get_all_sets(conn)).unwrap();*/
}

#[tauri::command]
fn get_all_sets(app_handle: AppHandle) -> Vec<Set> {
    app_handle.conn(|conn| database::get_all_sets(conn)).unwrap()
}

#[tauri::command]
fn drop_sets(set_ids: Vec<i32>, app_handle: AppHandle) {
    app_handle.conn(|conn| database::drop_sets(conn, set_ids).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState { conn: Default::default() })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            add_set, get_all_sets, drop_sets
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let conn = database::initialize_database(&app).expect("Database connection failed");
            *app_state.conn.lock().unwrap() = Some(conn);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
