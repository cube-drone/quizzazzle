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
use rusqlite::{Connection as SqlConnection, DatabaseName, params, Error as SqlError};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use crate::services::background_tick::RequiresBackgroundTick;
use crate::auth::model::UserId;
use crate::services::create_table::execute_and_eat_already_exists_errors;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDatabaseRaw {
    pub id: UserId,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDatabaseCreate{
    pub id: UserId,
    pub display_name: String,
    pub parent_id: Option<Uuid>,
    pub hashed_password: String,
    pub email: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub tags: Vec<String>,
}

impl UserDatabaseCreate{
    pub fn to_raw(self) -> UserDatabaseRaw{
        UserDatabaseRaw{
            id: self.id,
            display_name: self.display_name,
            parent_id: self.parent_id,
            hashed_password: self.hashed_password,
            email: self.email,
            thumbnail_url: self.thumbnail_url,
            is_verified: self.is_verified,
            is_admin: self.is_admin,
            tags: self.tags,
            opcount: 0,
            logincount: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone)]
pub struct UserService{
    cache: Arc<Cache<Uuid, UserDatabaseRaw>>,
    connection: Arc<Mutex<SqlConnection>>,
    options: UserServiceOptions,
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

const INSERT: &str = r#"INSERT INTO user (
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
const SELECT: &str = r#"SELECT
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
    FROM user WHERE id = ?1"#;

const PING: &str = "UPDATE user SET updated = CURRENT_TIMESTAMP WHERE id = ?1";
const DELETE: &str = "DELETE FROM user WHERE id = ?1";

const CREATE_TABLE_TAGS: &str = r#"CREATE TABLE IF NOT EXISTS user_tags (
    user_id UUID,
    tag TEXT,
    created INT NOT NULL,
    PRIMARY KEY (user_id, tag)
)"#;
const CREATE_INDEX_TAGS: &str = "CREATE INDEX IF NOT EXISTS user_tags ON user_tags (tag)";
const INSERT_TAG: &str = "INSERT INTO user_tags (user_id, tag, created) VALUES (?1, ?2, unixepoch())";
const SELECT_TAGS: &str = "SELECT tag FROM user_tags WHERE user_id = ?1";
const SELECT_ALL_USERS_MATCHING_TAG: &str = r#"SELECT
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
    FROM user WHERE id IN (SELECT user_id FROM user_tags WHERE tag = ?1)
    ORDER BY updated DESC
    LIMIT ?2
    OFFSET ?3"#;


impl UserService {
    pub fn new(options: UserServiceOptions) -> Result<Self>{
        let cache = Cache::builder()
            .max_capacity(100000)
            .time_to_idle(Duration::from_secs(86400))
            .build();

        let cache = Arc::new(cache);

        let sql_connection = Arc::new(Mutex::new(SqlConnection::open(format!("{}/user_{}.db", options.data_directory, options.name)).expect("Could not open User SQLite database")));
        Self::initialize(sql_connection.clone())?;

        Ok(UserService{cache, options, connection: sql_connection})
    }

    pub fn initialize(connection: Arc<Mutex<SqlConnection>>) -> Result<()> {
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_TABLE)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_UPDATED)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_CREATED)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_PARENT)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_EMAIL)?;

        execute_and_eat_already_exists_errors(connection.clone(), CREATE_TABLE_TAGS)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_TAGS)?;

        // Pragma Stuff
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to initialize user database"))?;
        // Set the journal mode and synchronous mode: WAL and normal
        // (WAL is write-ahead logging, which is faster and more reliable than the default rollback journal)
        // (normal synchronous mode is the best choice for WAL, and is the best tradeoff between speed and reliability)
        connection.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")?;
        connection.pragma_update(Some(DatabaseName::Main), "synchronous", "normal")?;

        Ok(())
    }

    fn create_sql(connection: Arc<Mutex<SqlConnection>>, user: UserDatabaseCreate) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(INSERT)?;
        statement.execute(params![
            user.id.to_uuid(),
            user.display_name,
            user.parent_id,
            user.hashed_password,
            user.email,
            user.thumbnail_url,
            user.is_verified,
            user.is_admin,
        ])?;

        for tag in user.tags {
            let mut statement = connection.prepare_cached(INSERT_TAG)?;
            statement.execute(params![user.id.to_uuid(), tag])?;
        }

        Ok(())
    }

    async fn async_create_sql(&self, user: UserDatabaseCreate) -> Result<()> {
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            Self::create_sql(connection, user)
        }).await??;

        Ok(())
    }

    pub async fn create_user(&self, user: UserDatabaseCreate) -> Result<()> {
        self.cache.insert(user.id.to_uuid(), user.clone().to_raw()).await;

        self.async_create_sql(user).await?;

        Ok(())
    }

    fn get_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<Option<UserDatabaseRaw>> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(SELECT)?;
        let mut rows = statement.query(params![user_id])?;

        match rows.next()?{
            Some(row) => {
                let created_at: Result<i64, SqlError> = row.get(10);
                let created_at = created_at.expect("Could not get created_at, even though it's a not null field") as i64;
                let updated_at: Result<i64, SqlError> = row.get(11);
                let updated_at = updated_at.expect("Could not get updated_at, even though it's a not null field") as i64;
                let created = DateTime::from_timestamp(created_at, 0).expect("Could not convert created_at to DateTime");
                let updated = DateTime::from_timestamp(updated_at, 0).expect("Could not convert updated_at to DateTime");

                Ok(Some(UserDatabaseRaw{
                    id: UserId::from_uuid(row.get(0)?),
                    display_name: row.get(1)?,
                    parent_id: row.get(2)?,
                    hashed_password: row.get(3)?,
                    email: row.get(4)?,
                    thumbnail_url: row.get(5)?,
                    is_verified: row.get(6)?,
                    is_admin: row.get(7)?,
                    tags: vec![],
                    opcount: row.get(8)?,
                    logincount: row.get(9)?,
                    created_at: created,
                    updated_at: updated,
                }))
            },
            None => Ok(None)
        }

    }

    async fn get_sql_async(&self, user_id: &Uuid) -> Result<Option<UserDatabaseRaw>> {
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        let user = tokio::task::spawn_blocking(move || {
            Self::get_sql(connection, &user_id)
        }).await??;

        Ok(user)
    }

    pub async fn get_user(&self, user_id: &UserId) -> Result<Option<UserDatabaseRaw>> {
        match self.cache.get(&user_id.to_uuid()).await{
            Some(cached) => Ok(Some(cached)),
            None => {
                match self.get_sql_async(&user_id.to_uuid()).await? {
                    Some(user) => {
                        self.cache.insert(user_id.to_uuid(), user.clone()).await;
                        Ok(Some(user))
                    },
                    None => Ok(None)
                }
            }
        }
    }
}

#[tokio::test]
async fn test_create_and_get(){
    let options = UserServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test".to_string(),
        cache_capacity: 100,
    };

    let service = UserService::new(options).unwrap();

    let creatable_user = UserDatabaseCreate{
        id: UserId::from_uuid(Uuid::new_v4()),
        display_name: "test".to_string(),
        parent_id: None,
        hashed_password: "test".to_string(),
        email: "test".to_string(),
        thumbnail_url: "test".to_string(),
        is_verified: false,
        is_admin: false,
        tags: vec!["test".to_string()],
    };

    service.create_user(creatable_user.clone()).await.unwrap();

    let user = service.get_user(&creatable_user.id).await.unwrap().unwrap();

    assert_eq!(user.id, creatable_user.id);
    assert_eq!(user.display_name, creatable_user.display_name);
    assert_eq!(user.parent_id, creatable_user.parent_id);
    assert_eq!(user.hashed_password, creatable_user.hashed_password);
    assert_eq!(user.email, creatable_user.email);
    assert_eq!(user.thumbnail_url, creatable_user.thumbnail_url);
    assert_eq!(user.is_verified, creatable_user.is_verified);
    assert_eq!(user.is_admin, creatable_user.is_admin);
    assert_eq!(user.tags, creatable_user.tags);
    assert_eq!(user.opcount, 0);
    assert_eq!(user.logincount, 0);
    assert_eq!(user.created_at.timestamp(), user.updated_at.timestamp());
}