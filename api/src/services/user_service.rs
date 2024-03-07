use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;

use anyhow::Result;
use futures::join;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, DatabaseName, params};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use crate::services::background_tick::RequiresBackgroundTick;

pub struct UserServiceOptions{
    pub cache_capacity: u64,
    pub data_directory: String,
}

pub struct UserDatabaseRaw {
    pub id: Uuid,
    pub display_name: String,
    pub parent_id: Option<Uuid>,
    pub hashed_password: String,
    pub email: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub tags: Vec<String>,
    pub opcount: i32,
    pub logincount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Clone)]
pub struct UserService{
    cache: Arc<Cache<Uuid, UserDatabaseRaw>>,
    connection: Arc<Mutex<SqlConnection>>,
}

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS user (
    id UUID PRIMARY KEY,
    display_name TEXT NOT NULL,
    parent_id UUID,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL,
    thumbnail_url TEXT,
    is_verified BOOLEAN NOT NULL,
    is_admin BOOLEAN NOT NULL,
    opcount INT NOT NULL DEFAULT 0,
    logincount INT NOT NULL DEFAULT 0,
    created INT NOT NULL,
    updated INT NOT NULL
)"#;
const CREATE_INDEX_UPDATED: &str = "CREATE INDEX IF NOT EXISTS user_updated ON user (updated)";
const CREATE_INDEX_CREATED: &str = "CREATE INDEX IF NOT EXISTS user_created ON user (created)";
const CREATE_INDEX_PARENT: &str = "CREATE INDEX IF NOT EXISTS user_parent ON user (parent_id)";
const CREATE_INDEX_EMAIL: &str = "CREATE INDEX IF NOT EXISTS user_email ON user (email)";

const INSERT: &str = r#"INSERT INTO tokens (
    id,
    display_name,
    parent_id,
    hashed_password,
    email,
    thumbnail_url,
    is_verified,
    is_admin,
    opcount,
    logincount,
    created,
    updated
) VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5,
    ?6,
    ?7,
    ?8,
    0,
    0,
    unixepoch(),
    unixepoch()
)"#;
//const UPDATE: &str = "UPDATE user SET value = ?2, updated = unixepoch() WHERE id = ?1";
const SELECT: &str = "SELECT id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, is_admin, opcount, logincount, created, updated FROM tokens WHERE id = ?1";
const PING: &str = "UPDATE user SET updated = CURRENT_TIMESTAMP WHERE id = ?1";
const DELETE: &str = "DELETE FROM user WHERE id = ?1";


impl UserService {
    pub fn new(options: UserServiceOptions) -> UserService{
        let cache = Cache::builder()
            .max_capacity(100000)
            .time_to_idle(Duration::from_secs(86400))
            .build();

        let connection = SqlConnection::open_in_memory().unwrap();
        let connection = Arc::new(Mutex::new(connection));
        let cache = Arc::new(cache);

        let sql_connection = SqlConnection::open(format!("{}/user.db", options.data_directory)).expect("Could not open User SQLite database");
        self.initialize().expect("Could not initialize User SQLite database");

        UserService{cache, connection}
    }

    pub fn initialize(&self) -> Result<()> {
        let mut connection = self.connection.lock()?;
        connection.execute(CREATE_TABLE, [])?;
        connection.execute(CREATE_INDEX_UPDATED, [])?;
        connection.execute(CREATE_INDEX_CREATED, [])?;
        connection.execute(CREATE_INDEX_PARENT, [])?;
        connection.execute(CREATE_INDEX_EMAIL, [])?;
    }

}