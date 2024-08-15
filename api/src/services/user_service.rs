use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashSet;
use std::net::IpAddr;

use anyhow::Result;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, DatabaseName, params, Error as SqlError};

use serde::{Serialize, Deserialize};

//use crate::services::background_tick::RequiresBackgroundTick;
use crate::auth::model::UserId;
use crate::services::create_table::execute_and_eat_already_exists_errors;
use crate::services::user_ip_service::UserIpService;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct UserServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub drop_table_on_start: bool,
}

#[derive(Clone, PartialEq)]
pub struct UserDatabaseRaw {
    pub id: UserId,
    pub display_name: String,
    pub parent_id: Option<UserId>,
    pub hashed_password: String,
    pub email: String,
    pub email_domain: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub tags: HashSet<String>,
    pub opcount: i32,
    pub logincount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

const INVITE_CODE_REGENERATION_TIME_MS: i64 = 86400 * 1000 * 4; // 4 days

fn calculate_available_user_invites(created_at: DateTime<Utc>) -> i32 {
    let time_since_creation = Utc::now() - created_at;
    let time_in_ms = time_since_creation.num_milliseconds() as f64;
    let invite_codes = time_in_ms as f64 / INVITE_CODE_REGENERATION_TIME_MS as f64;
    let n_invite_codes: i32 = invite_codes.floor() as i32;
    n_invite_codes
}

#[tokio::test]
async fn test_user_invites(){
    let now = Utc::now();
    let a_few_days_ago = now - (chrono::Duration::milliseconds(INVITE_CODE_REGENERATION_TIME_MS * 3) + chrono::Duration::milliseconds(1000));

    let available_user_invites_now = calculate_available_user_invites(now);
    let available_user_invites_days_ago = calculate_available_user_invites(a_few_days_ago);

    assert_eq!(available_user_invites_now, 0);
    assert_eq!(available_user_invites_days_ago, 3);
}

impl UserDatabaseRaw {
    pub fn available_user_invites(&self) -> i32 {
        if self.is_admin {
            return 10000;
        }
        if self.tags.contains(&"unlimited_invites".to_string()) {
            return 10000;
        }
        calculate_available_user_invites(self.created_at)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDatabaseCreate{
    pub id: UserId,
    pub display_name: String,
    pub parent_id: Option<UserId>,
    pub hashed_password: String,
    pub email: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub tags: HashSet<String>,
}

impl UserDatabaseCreate{
    pub fn to_raw(self) -> UserDatabaseRaw{
        let domain = self.email.split('@').collect::<Vec<&str>>()[1].to_string();
        UserDatabaseRaw{
            id: self.id,
            display_name: self.display_name,
            parent_id: self.parent_id,
            hashed_password: self.hashed_password,
            email: self.email,
            email_domain: domain,
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
    user_ip_service: UserIpService,
    _options: UserServiceOptions,
}

const CREATE_TABLE: &str = r#"CREATE TABLE IF NOT EXISTS user (
    id UUID PRIMARY KEY,
    display_name TEXT NOT NULL,
    parent_id UUID,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    email_domain TEXT NOT NULL,
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
const CREATE_INDEX_EMAIL_DOMAIN: &str = "CREATE INDEX IF NOT EXISTS user_email_domain ON user (email_domain)";

const INSERT: &str = r#"INSERT INTO user (
    id,
    display_name,
    parent_id,
    hashed_password,
    email,
    email_domain,
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
    ?9,
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
    email_domain,
    thumbnail_url,
    is_verified,
    is_admin,
    opcount,
    logincount,
    created,
    updated
    FROM user WHERE id = ?1"#;
const SELECT_EXISTS: &str = "SELECT id FROM user WHERE id = ?1";
const SELECT_EMAIL: &str = "SELECT id FROM user WHERE email = ?1";

//const PING: &str = "UPDATE user SET updated = unixepoch() WHERE id = ?1";
const VERIFY: &str = "UPDATE user SET is_verified = true, updated = unixepoch() WHERE id = ?1";
const PASSWORD_CHANGE: &str = "UPDATE user SET hashed_password = ?2, updated = unixepoch() WHERE id = ?1";

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
/*
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
 */


impl UserService {
    pub fn new(options: UserServiceOptions) -> Result<Self>{
        let cache = Cache::builder()
            .max_capacity(options.cache_capacity)
            .time_to_idle(Duration::from_secs(options.expiry_seconds))
            .build();

        let cache = Arc::new(cache);

        let sql_connection = Arc::new(Mutex::new(SqlConnection::open(format!("{}/user_{}.db", options.data_directory, options.name)).expect("Could not open User SQLite database")));

        let drop_table = options.drop_table_on_start;

        if drop_table {
            let connection = sql_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to drop table"))?;
            connection.execute("DROP TABLE IF EXISTS user", [])?;
            connection.execute("DROP TABLE IF EXISTS user_tags", [])?;
        }

        Self::initialize(sql_connection.clone())?;

        Ok(UserService{cache, _options: options, connection: sql_connection.clone(), user_ip_service: UserIpService::new(sql_connection.clone(), drop_table)?})
    }

    pub fn initialize(connection: Arc<Mutex<SqlConnection>>) -> Result<()> {
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_TABLE)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_UPDATED)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_CREATED)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_PARENT)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_EMAIL)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_EMAIL_DOMAIN)?;

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
            user.parent_id.map(|id| id.to_uuid()),
            user.hashed_password,
            user.email,
            user.email.split('@').collect::<Vec<&str>>()[1],
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
                let created_at: Result<i64, SqlError> = row.get(11);
                let created_at = created_at.expect("Could not get created_at, even though it's a not null field") as i64;
                let updated_at: Result<i64, SqlError> = row.get(12);
                let updated_at = updated_at.expect("Could not get updated_at, even though it's a not null field") as i64;
                let created = DateTime::from_timestamp(created_at, 0).expect("Could not convert created_at to DateTime");
                let updated = DateTime::from_timestamp(updated_at, 0).expect("Could not convert updated_at to DateTime");

                let mut statement = connection.prepare_cached(SELECT_TAGS)?;
                let mut rows = statement.query(params![user_id])?;

                let mut tags = HashSet::new();
                while let Some(row) = rows.next()?{
                    tags.insert(row.get(0)?);
                }

                let parent_id = match row.get(2)?{
                    Some(id) => Some(UserId::from_uuid(id)),
                    None => None
                };

                Ok(Some(UserDatabaseRaw{
                    id: UserId::from_uuid(row.get(0)?),
                    display_name: row.get(1)?,
                    parent_id,
                    hashed_password: row.get(3)?,
                    email: row.get(4)?,
                    email_domain: row.get(5)?,
                    thumbnail_url: row.get(6)?,
                    is_verified: row.get(7)?,
                    is_admin: row.get(8)?,
                    tags,
                    opcount: row.get(9)?,
                    logincount: row.get(10)?,
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

    pub fn user_exists(&self, user_id: &UserId) -> Result<bool> {
        match self.cache.contains_key(&user_id.to_uuid()){
            true => Ok(true),
            false => {
                let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
                let mut statement = connection.prepare_cached(SELECT_EXISTS)?;
                let mut rows = statement.query(params![user_id.to_uuid()])?;

                match rows.next()?{
                    Some(_) => Ok(true),
                    None => Ok(false)
                }
            }
        }
    }

    fn get_user_by_email_sql(&self, email: &str) -> Result<Option<Uuid>> {
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(SELECT_EMAIL)?;
        let mut rows = statement.query(params![email])?;

        match rows.next()?{
            Some(row) => {
                let user_id: Uuid = row.get(0)?;
                Ok(Some(user_id))
            },
            None => Ok(None)
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserDatabaseRaw>> {
        match self.get_user_by_email_sql(email)?{
            Some(user_id) => {
                self.get_user(&UserId::from_uuid(user_id)).await
            }
            None => Ok(None)
        }
    }

    fn verify_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(VERIFY)?;
        statement.execute(params![user_id])?;

        Ok(())
    }

    async fn verify_sql_async(&self, user_id: &UserId) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.to_uuid();
        tokio::task::spawn_blocking(move || {
            Self::verify_sql(connection, &user_id)
        }).await??;

        Ok(())
    }

    pub async fn verify_user(&self, user_id: &UserId) -> Result<()> {
        if !self.user_exists(user_id)? {
            return Err(anyhow::anyhow!("User does not exist"));
        }

        self.verify_sql_async(user_id).await?;

        // update dat cache
        match self.cache.get(&user_id.to_uuid()).await{
            Some(mut user) => {
                user.is_verified = true;
                self.cache.insert(user_id.to_uuid(), user).await;
            },
            None => {}
        }

        Ok(())
    }

    fn password_change_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid, new_password: &String) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(PASSWORD_CHANGE)?;
        statement.execute(params![user_id, new_password])?;

        Ok(())
    }

    async fn password_change_sql_async(&self, user_id: &UserId, new_password: &String) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.to_uuid();
        let new_password = new_password.clone();
        tokio::task::spawn_blocking(move || {
            Self::password_change_sql(connection, &user_id, &new_password)
        }).await??;

        Ok(())
    }

    pub async fn change_password(&self, user_id: &UserId, new_password: &String) -> Result<()> {
        if !self.user_exists(user_id)? {
            return Err(anyhow::anyhow!("User does not exist"));
        }
        self.password_change_sql_async(user_id, new_password).await?;

        // update dat cache
        match self.cache.get(&user_id.to_uuid()).await{
            Some(mut user) => {
                user.hashed_password = new_password.clone();
                self.cache.insert(user_id.to_uuid(), user).await;
            },
            None => {}
        }

        Ok(())
    }

    pub async fn clear_cache(&self, user_id: &UserId) -> Result<()> {
        self.cache.remove(&user_id.to_uuid()).await;
        Ok(())
    }


    fn delete_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<()> {
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(DELETE)?;
        statement.execute(params![user_id])?;

        Ok(())
    }

    async fn delete_sql_async(&self, user_id: &UserId) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.to_uuid();
        tokio::task::spawn_blocking(move || {
            Self::delete_sql(connection, &user_id)
        }).await??;

        Ok(())
    }

    pub async fn delete_user(&self, user_id: &UserId) -> Result<()> {
        self.delete_sql_async(user_id).await?;
        self.user_ip_service.delete_user(&user_id.to_uuid()).await?;
        self.cache.remove(&user_id.to_uuid()).await;
        Ok(())
    }


    pub async fn increment_opcount(&self, user_id: &UserId) -> Result<()> {
        // TODO: OPCOUNT
        match self.cache.get(&user_id.to_uuid()).await{
            Some(mut user) => {
                user.opcount += 1;
                self.cache.insert(user_id.to_uuid(), user).await;
            },
            None => {}
        }

        Ok(())
    }

    pub async fn increment_logincount(&self, user_id: &UserId) -> Result<()> {
        // TODO: LOGINCOUNT
        if !self.user_exists(user_id)? {
            return Err(anyhow::anyhow!("User does not exist"));
        }

        match self.cache.get(&user_id.to_uuid()).await{
            Some(mut user) => {
                user.logincount += 1;
                self.cache.insert(user_id.to_uuid(), user).await;
            },
            None => {}
        }

        Ok(())
    }

    pub async fn set_user_ip(
        &self,
        user_id: &UserId,
        ip: IpAddr
    ) -> Result<()> {
        if !self.user_exists(user_id)? {
            return Err(anyhow::anyhow!("User does not exist"));
        }
        self.user_ip_service.set_user_ip(user_id.to_uuid(), ip).await?;

        Ok(())
    }

    pub fn user_has_used_ip(&self, user_id: &UserId, ip: &IpAddr) -> Result<bool> {
        self.user_ip_service.user_has_used_ip(&user_id.to_uuid(), ip)
    }

    pub async fn delete_ip(&self, user_id: &UserId, ip: &IpAddr) -> Result<()> {
        self.user_ip_service.delete_ip(&user_id.to_uuid(), ip.clone()).await
    }

}

#[tokio::test]
async fn test_create_and_get(){
    let options = UserServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test_create".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true
    };

    let service = UserService::new(options).unwrap();

    let email = format!("{}@gmail.com", Uuid::new_v4());

    let creatable_user = UserDatabaseCreate{
        id: UserId::from_uuid(Uuid::new_v4()),
        display_name: "test".to_string(),
        parent_id: None,
        hashed_password: "test".to_string(),
        email: email.clone(),
        thumbnail_url: "test".to_string(),
        is_verified: false,
        is_admin: false,
        tags: HashSet::from(["test".to_string(), "hello".to_string(), "world".to_string()]),
    };

    assert!(!service.user_exists(&creatable_user.id).unwrap());

    service.create_user(creatable_user.clone()).await.unwrap();

    assert!(service.user_exists(&creatable_user.id).unwrap());

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

    service.verify_user(&creatable_user.id).await.unwrap();
    service.change_password(&creatable_user.id, &"anus".to_string()).await.unwrap();

    let user = service.get_user_by_email(&email).await.unwrap().unwrap();
    assert_eq!(user.id, creatable_user.id);
    assert_eq!(user.display_name, creatable_user.display_name);
    assert_eq!(user.parent_id, creatable_user.parent_id);
    assert_eq!(user.hashed_password, "anus".to_string());
    assert_eq!(user.email, creatable_user.email);
    assert_eq!(user.thumbnail_url, creatable_user.thumbnail_url);
    assert!(user.is_verified);
    assert_eq!(user.is_admin, creatable_user.is_admin);
    assert_eq!(user.tags, creatable_user.tags);
    assert_eq!(user.opcount, 0);
    assert_eq!(user.logincount, 0);
    assert_eq!(user.created_at.timestamp(), user.updated_at.timestamp());

    service.clear_cache(&creatable_user.id).await.unwrap();

    // this all still works even post cache-clear
    let user = service.get_user_by_email(&email).await.unwrap().unwrap();
    assert_eq!(user.id, creatable_user.id);
    assert_eq!(user.display_name, creatable_user.display_name);
    assert_eq!(user.parent_id, creatable_user.parent_id);
    assert_eq!(user.hashed_password, "anus".to_string());
    assert_eq!(user.email, creatable_user.email);
    assert_eq!(user.thumbnail_url, creatable_user.thumbnail_url);
    assert!(user.is_verified);
    assert_eq!(user.is_admin, creatable_user.is_admin);
    assert_eq!(user.tags, creatable_user.tags);
    assert_eq!(user.opcount, 0);
    assert_eq!(user.logincount, 0);
    assert_eq!(user.created_at.timestamp(), user.updated_at.timestamp());

    service.delete_user(&creatable_user.id).await.unwrap();

    assert!(!service.user_exists(&creatable_user.id).unwrap());
    assert!(service.get_user(&creatable_user.id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_ip_stuff(){
    let options = UserServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test_ip".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: false
    };

    let service = UserService::new(options).unwrap();

    let email = format!("{}@gmail.com", Uuid::new_v4());

    let creatable_user = UserDatabaseCreate{
        id: UserId::from_uuid(Uuid::new_v4()),
        display_name: "toast".to_string(),
        parent_id: None,
        hashed_password: "test".to_string(),
        email: email.clone(),
        thumbnail_url: "test".to_string(),
        is_verified: false,
        is_admin: false,
        tags: HashSet::from(["test".to_string(), "hello".to_string(), "world".to_string()]),
    };

    service.create_user(creatable_user.clone()).await.unwrap();

    let ip = IpAddr::from_str("192.168.1.1").unwrap();

    match service.user_has_used_ip(&creatable_user.id, &ip).unwrap(){
        true => panic!("User has used ip"),
        false => {}
    }

    service.set_user_ip(&creatable_user.id, ip).await.unwrap();

    match service.user_has_used_ip(&creatable_user.id, &ip).unwrap(){
        true => {}
        false => panic!("User has not used ip"),
    }
}