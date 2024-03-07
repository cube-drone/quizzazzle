use std::sync::{Arc, Mutex};
use anyhow::Result;
use rusqlite::Connection as SqlConnection;

///
/// We know that CREATE TABLE IF NOT EXISTS will usually fail (the table will already exist), so we eat the error
///
pub fn execute_and_eat_already_exists_errors(connection: Arc<Mutex<SqlConnection>>, sql: &str) -> Result<()> {
    let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to initialize user database"))?;
    match connection.execute(sql, []){
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("there is already") {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Could not execute SQL: {}", e))
            }
        }
    }
}