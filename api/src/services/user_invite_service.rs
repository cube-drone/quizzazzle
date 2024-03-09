use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use anyhow::Result;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, DatabaseName, params, Error as SqlError};
use chrono::{DateTime, Utc};

//use crate::services::background_tick::RequiresBackgroundTick;
use crate::auth::model::{UserId, InviteCode};
use crate::services::create_table::execute_and_eat_already_exists_errors;


#[derive(Clone, Serialize, Deserialize)]
pub struct UserInviteRaw {
    pub invite_code: InviteCode,
    pub user_id: UserId,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

pub struct UserInviteServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub drop_table_on_start: bool,
}

#[derive(Clone)]
pub struct UserInviteService{
    connection: Arc<Mutex<SqlConnection>>,
    token_cache: Arc<Mutex<Cache<Uuid, UserInviteRaw>>>,
    user_tokens_cache: Arc<Mutex<Cache<Uuid, HashSet<Uuid>>>>
}

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    is_used BOOLEAN NOT NULL,
    created INT NOT NULL,
    used_at INT
)"#;
const CREATE_INDEX_USER: &str = "CREATE INDEX IF NOT EXISTS token_user_index ON tokens (user_id)";

const INSERT: &str = "INSERT INTO tokens (id, user_id, is_used, created) VALUES (?1, ?2, false, unixepoch());";
const SELECT_ONE: &str = "SELECT id, user_id, is_used, created, used_at FROM tokens WHERE id = ?1;";
const USE_TOKEN: &str = "UPDATE tokens SET is_used = true, used_at = unixepoch() WHERE id = ?1;";
const SELECT_MANY: &str = "SELECT id, user_id, is_used, created, used_at FROM tokens WHERE user_id = ?1 ORDER BY created;";
const COUNT: &str = "SELECT COUNT(*) FROM tokens WHERE user_id = ?1;";
const DELETE: &str = "DELETE FROM tokens WHERE id = ?1;";

impl UserInviteService{
    pub fn new(options: UserInviteServiceOptions) -> Result<Self> {
        let token_cache: Cache<Uuid, UserInviteRaw> = Cache::builder()
            .max_capacity(options.cache_capacity)
            .time_to_idle(Duration::from_secs(options.expiry_seconds))
            .build();

        let user_tokens_cache: Cache<Uuid, HashSet<Uuid>> = Cache::builder()
            .max_capacity(options.cache_capacity)
            .time_to_idle(Duration::from_secs(options.expiry_seconds))
            .build();

        let sql_connection = Arc::new(Mutex::new(SqlConnection::open(format!("{}/invites_{}.db", options.data_directory, options.name)).expect("Could not open Invite SQLite database")));

        let drop_table = options.drop_table_on_start;

        if drop_table {
            let connection = sql_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to drop table"))?;
            connection.execute("DROP TABLE IF EXISTS tokens", [])?;
        }

        Self::initialize(sql_connection.clone())?;

        Ok(Self {
            connection: sql_connection,
            token_cache: Arc::new(Mutex::new(token_cache)),
            user_tokens_cache: Arc::new(Mutex::new(user_tokens_cache))
        })
    }

    pub fn initialize(connection: Arc<Mutex<SqlConnection>>) -> Result<()> {
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_TABLE)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_USER)?;

        // Pragma Stuff
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to initialize user database"))?;
        // Set the journal mode and synchronous mode: WAL and normal
        // (WAL is write-ahead logging, which is faster and more reliable than the default rollback journal)
        // (normal synchronous mode is the best choice for WAL, and is the best tradeoff between speed and reliability)
        connection.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")?;
        connection.pragma_update(Some(DatabaseName::Main), "synchronous", "normal")?;

        Ok(())
    }

    fn create_invite_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid, invite_code: &Uuid) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(INSERT)?;
        statement.execute(params![invite_code, user_id])?;

        Ok(())
    }

    async fn create_invite_sql_async(&self, user_id: Uuid, invite_code: Uuid) -> Result<()> {
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            Self::create_invite_sql(connection, &user_id, &invite_code)
        }).await??;

        Ok(())
    }

    pub async fn create_invite(&self, user_id: &UserId) -> Result<InviteCode> {
        let invite_code = Uuid::new_v4();
        self.create_invite_sql_async(user_id.to_uuid(), invite_code).await?;

        Ok(InviteCode::from_uuid(invite_code))
    }

    fn get_invite_sql(connection: Arc<Mutex<SqlConnection>>, invite_code: &Uuid) -> Result<Option<UserInviteRaw>> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(SELECT_ONE)?;
        let mut rows = statement.query(params![invite_code])?;

        match rows.next()?{
            Some(row) => {
                let id: Uuid = row.get(0)?;
                let user_id: Uuid = row.get(1)?;
                let is_used: bool = row.get(2)?;

                let created_at: Result<i64, SqlError> = row.get(3);
                let created_at = created_at.expect("Could not get created_at, even though it's a not null field") as i64;
                let created = match DateTime::from_timestamp(created_at, 0){
                    Some(dt) => dt,
                    None => return Err(anyhow::anyhow!("Could not convert created_at to DateTime"))
                };
                let used_at: Result<Option<i64>, SqlError> = row.get(4);
                let used_at = used_at?;
                let used = match used_at{
                    Some(timestamp) => match DateTime::from_timestamp(timestamp, 0){
                        Some(dt) => Some(dt),
                        None => return Err(anyhow::anyhow!("Could not convert used_at to DateTime"))
                    },
                    None => None
                };

                Ok(Some(UserInviteRaw{
                    invite_code: InviteCode::from_uuid(id),
                    user_id: UserId::from_uuid(user_id),
                    is_used,
                    created_at: created,
                    used_at: used
                }))
            },
            None => Ok(None)
        }
    }

    async fn get_invite_sql_async(&self, invite_code: Uuid) -> Result<Option<UserInviteRaw>> {
        let connection = self.connection.clone();
        let invite: Option<UserInviteRaw> = tokio::task::spawn_blocking(move || {
            Self::get_invite_sql(connection, &invite_code)
        }).await??;

        Ok(invite)
    }

    pub async fn get_invite(&self, invite_code: &InviteCode) -> Result<Option<UserInviteRaw>> {
        let invite = self.get_invite_sql_async(invite_code.to_uuid()).await?;

        Ok(invite)
    }

    pub async fn invite_exists(&self, invite_code: &InviteCode) -> Result<bool> {
        let invite = self.get_invite(invite_code).await.unwrap();

        match invite{
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    fn use_invite_sql(connection: Arc<Mutex<SqlConnection>>, invite_code: &Uuid) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to use token"))?;
        let mut statement = connection.prepare_cached(USE_TOKEN)?;
        statement.execute(params![invite_code])?;

        Ok(())
    }

    async fn use_invite_sql_async(&self, invite_code: Uuid) -> Result<()> {
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            Self::use_invite_sql(connection, &invite_code)
        }).await??;

        Ok(())
    }

    pub async fn use_invite(&self, invite_code: &InviteCode) -> Result<()> {
        self.use_invite_sql_async(invite_code.to_uuid()).await?;

        Ok(())
    }

    fn get_invites_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<Vec<UserInviteRaw>> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get tokens"))?;
        let mut statement = connection.prepare_cached(SELECT_MANY)?;
        let mut rows = statement.query(params![user_id])?;
        let mut tokens = Vec::new();

        while let Some(row) = rows.next()?{
            let id: Uuid = row.get(0)?;
            let user_id: Uuid = row.get(1)?;
            let is_used: bool = row.get(2)?;

            let created_at: Result<i64, SqlError> = row.get(3);
            let created_at = created_at.expect("Could not get created_at, even though it's a not null field") as i64;
            let created = match DateTime::from_timestamp(created_at, 0){
                Some(dt) => dt,
                None => return Err(anyhow::anyhow!("Could not convert created_at to DateTime"))
            };
            let used_at: Result<Option<i64>, SqlError> = row.get(4);
            let used_at = used_at?;
            let used = match used_at{
                Some(timestamp) => match DateTime::from_timestamp(timestamp, 0){
                    Some(dt) => Some(dt),
                    None => return Err(anyhow::anyhow!("Could not convert used_at to DateTime"))
                },
                None => None
            };

            tokens.push(UserInviteRaw{
                invite_code: InviteCode::from_uuid(id),
                user_id: UserId::from_uuid(user_id),
                is_used,
                created_at: created,
                used_at: used
            });
        }

        Ok(tokens)
    }

    async fn get_invites_sql_async(&self, user_id: Uuid) -> Result<Vec<UserInviteRaw>> {
        let connection = self.connection.clone();
        let tokens: Vec<UserInviteRaw> = tokio::task::spawn_blocking(move || {
            Self::get_invites_sql(connection, &user_id)
        }).await??;

        Ok(tokens)
    }

    pub async fn get_invites(&self, user_id: &UserId) -> Result<Vec<UserInviteRaw>> {
        let tokens = self.get_invites_sql_async(user_id.to_uuid()).await?;

        Ok(tokens)
    }

    fn count_invites_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<i32> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to count tokens"))?;
        let mut statement = connection.prepare_cached(COUNT)?;
        let mut rows = statement.query(params![user_id])?;
        let mut count = 0;

        while let Some(row) = rows.next()?{
            count = row.get(0)?;
        }

        Ok(count)
    }

    async fn count_invites_sql_async(&self, user_id: Uuid) -> Result<i32> {
        let connection = self.connection.clone();
        let count: i32 = tokio::task::spawn_blocking(move || {
            Self::count_invites_sql(connection, &user_id)
        }).await??;

        Ok(count)
    }

    pub async fn count_invites(&self, user_id: &UserId) -> Result<i32> {
        Ok(self.count_invites_sql_async(user_id.to_uuid()).await?)
    }

    fn delete_invite_sql(connection: Arc<Mutex<SqlConnection>>, invite_code: &Uuid) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token"))?;
        let mut statement = connection.prepare_cached(DELETE)?;
        statement.execute(params![invite_code])?;

        Ok(())
    }

    async fn delete_invite_sql_async(&self, invite_code: Uuid) -> Result<()> {
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_invite_sql(connection, &invite_code)
        }).await??;

        Ok(())
    }

    pub async fn delete_invite(&self, invite_code: &InviteCode) -> Result<()> {
        self.delete_invite_sql_async(invite_code.to_uuid()).await?;

        Ok(())
    }

}

#[tokio::test]
async fn test_invite(){
    let options = UserInviteServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test7".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
    };

    let service = UserInviteService::new(options).unwrap();

    let user_id = UserId::from_uuid(Uuid::new_v4());

    service.create_invite(&user_id).await.unwrap();
    service.create_invite(&user_id).await.unwrap();
    service.create_invite(&user_id).await.unwrap();

    let count = service.count_invites(&user_id).await.unwrap();
    assert_eq!(count, 3);

    let invites = service.get_invites(&user_id).await.unwrap();

    assert_eq!(invites.len(), 3);
    assert_eq!(invites[0].user_id, user_id);
    assert_eq!(invites[0].is_used, false);
    assert!(invites[0].used_at.is_none());

    let invite = service.get_invite(&invites[0].invite_code).await.unwrap().unwrap();

    assert_eq!(invite.user_id, user_id);

    service.use_invite(&invites[0].invite_code).await.unwrap();
    service.use_invite(&invites[2].invite_code).await.unwrap();

    service.create_invite(&user_id).await.unwrap();

    let invites = service.get_invites(&user_id).await.unwrap();

    assert_eq!(invites.len(), 4);
    assert_eq!(invites[0].is_used, true);
    assert_eq!(invites[1].is_used, false);
    assert_eq!(invites[2].is_used, true);
    assert!(invites[0].used_at.is_some());
    assert!(invites[2].used_at.is_some());

    let count = service.count_invites(&user_id).await.unwrap();
    assert_eq!(count, 4);

    service.delete_invite(&invites[1].invite_code).await.unwrap();
    let count = service.count_invites(&user_id).await.unwrap();
    assert_eq!(count, 3);
}
