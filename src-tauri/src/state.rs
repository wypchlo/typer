use rusqlite::Connection;
use tauri::{AppHandle, State, Manager};

pub struct AppState {
    pub conn: std::sync::Mutex<Option<Connection>> 
}

pub trait ServiceAccess {
    fn conn<F, TResult>(&self, operation: F) -> TResult
        where F: FnOnce(&Connection) -> TResult;

    fn conn_mut<F, TResult>(&self, operation: F) -> TResult
        where F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn conn<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
        let app_state: State<AppState> = self.state();
        let conn_guard = app_state.conn.lock().unwrap();
        let conn = conn_guard.as_ref().unwrap();

        operation(conn)
    }

    fn conn_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
       let app_state: State<AppState> = self.state();
       let mut conn_guard = app_state.conn.lock().unwrap();
       let conn = conn_guard.as_mut().unwrap();

       operation(conn)
    }
}
