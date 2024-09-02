use rusqlite::{Connection, named_params}; use tauri::{App, Manager};
use std::fs;
use crate::dbstructs::{Set, NewSet};

const CURRENT_DB_VERSION: u32 = 2;

pub fn initialize_database(app: &App) -> Result<Connection, rusqlite::Error> {
    let app_dir = app.path().app_data_dir().expect("App data directory should exist");
    //app_dir is the directory with the app name inside of the user appdatafolder
    fs::create_dir_all(&app_dir).expect("Project app data directory should be created");
    let db_path = app_dir.join("typer.sqlite3");

    let mut conn = Connection::open(db_path)?;

    let mut user_pragma = conn.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| {
        Ok(row.get(0)?) 
    })?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut conn, existing_user_version)?;
    Ok(conn)
}

pub fn upgrade_database_if_needed(conn: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        conn.pragma_update(None, "journal_mode", "WAL")?; 

        let tx = conn.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;
        
        tx.execute_batch("
            CREATE TABLE IF NOT EXISTS sets
            (
                id INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_date TEXT NOT NULL
            );
        ")?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_set(set: NewSet, conn: &Connection) -> Result<(), rusqlite::Error> {
   let mut statement = conn.prepare("INSERT INTO sets (name, description, created_date) VALUES (@name, @description, @created_date)")?;
   let current_date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

   match statement.execute(named_params! { 
       "@name": set.name, 
       "@description": set.description, 
       "@created_date": current_date 
   }) {
        Ok(_) => Ok(()),
        Err(err) => Err(rusqlite::Error::from(err))
    }
}

pub fn get_all_sets(conn: &Connection) -> Result<Vec<Set>, rusqlite::Error> {
    let mut statement = conn.prepare("SELECT * FROM sets")?;
    let mut rows = statement.query([])?;
    let mut sets: Vec<Set> = Vec::new();
    
    while let Some(row) = rows.next()? {
        let id = row.get("id")?;
        let name = row.get("name")?;
        let description = row.get("description")?;
        let created_date = row.get("created_date")?;
        let new_set = Set{id, name, description, created_date};

        sets.push(new_set);
    }

    Ok(sets)
}

pub fn drop_sets(conn: &Connection, set_ids: Vec<i32>) -> Result<(), rusqlite::Error> {
    let mut statement = conn.prepare("DELETE FROM sets WHERE sets.id IN (@ids)")?;
    let mut ids = Vec::<String>::new();
    for id in set_ids.into_iter() { ids.push(id.to_string()); }

    match statement.execute(named_params! { "@ids": ids.join(", ") }) {
        Ok(_) => Ok(()),
        Err(err) => Err(rusqlite::Error::from(err))
    }
}
