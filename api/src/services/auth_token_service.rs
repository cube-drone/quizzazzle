#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;
use futures::try_join;

use anyhow::Result;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use rusqlite::{Connection as SqlConnection, DatabaseName};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

const DROP: &str = "DROP TABLE IF EXISTS list_tokens";
const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS list_tokens (user_id TEXT NOT NULL, token TEXT NOT NULL, created INTEGER NOT NULL, updated INTEGER NOT NULL)";
const CREATE_INDEX: &str = "CREATE INDEX IF NOT EXISTS user_id_index ON list_tokens (user_id)";
const CREATE_INDEX_CREATED: &str = "CREATE INDEX IF NOT EXISTS created_index ON list_tokens (created)";
const CREATE_INDEX_UPDATED: &str = "CREATE INDEX IF NOT EXISTS updated_index ON list_tokens (updated)";
const INSERT: &str = "INSERT INTO list_tokens (user_id, token, created, updated) VALUES (?1, ?2, unixepoch(), unixepoch())";
const SELECT: &str = "SELECT token, updated FROM list_tokens WHERE user_id = ?1";
const PING: &str = "UPDATE list_tokens SET updated = unixepoch() WHERE user_id = ?1 AND token = ?2";
const DELETE: &str = "DELETE FROM list_tokens WHERE user_id = ?1 AND token = ?2";
const DELETE_ALL: &str = "DELETE FROM list_tokens WHERE user_id = ?1";
const DELETE_IDLE: &str = "DELETE FROM list_tokens WHERE updated < ?1";

use crate::services::background_tick::RequiresBackgroundTick;
use crate::services::disposable_token_service::{DisposableTokenService, DisposableTokenServiceOptions};
use crate::services::timestamp_sorted_list_cache::TimestampSortedListCache;

pub trait HasUserId{
    fn user_id(&self) -> Uuid;
}

#[derive(Clone)]
pub struct AuthTokenService<T: 'static> where T: Serialize + DeserializeOwned + Clone + Sync + Send + HasUserId{
    disposable_token_service: DisposableTokenService<T>,
    timestamp_sorted_list_cache: Arc<TimestampSortedListCache<Uuid>>,
    connection: Arc<Mutex<SqlConnection>>,
    options: AuthTokenServiceOptions,
}

// an AuthTokenService works like a DisposableTokenService, but:
//  you can look up all of the tokens that have been issued to a single user
//  you can revoke all of the tokens that have been issued to a single user
//  the number of tokens that can be issued to a single user is limited
//  if a user has too many tokens, the oldest tokens are automatically revoked

#[derive(Clone)]
pub struct AuthTokenServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub drop_table_on_start: bool,
    pub max_tokens_per_user: usize,
}

impl AuthTokenServiceOptions{
    pub fn to_disposable_token_service_options(&self) -> DisposableTokenServiceOptions{
        DisposableTokenServiceOptions{
            data_directory: self.data_directory.clone(),
            name: self.name.clone(),
            cache_capacity: self.cache_capacity,
            expiry_seconds: self.expiry_seconds,
            get_refreshes_expiry: true,
            probability_of_refresh: 0.1,
            drop_table_on_start: self.drop_table_on_start,
        }
    }
}

impl<T> AuthTokenService<T> where T: Serialize + DeserializeOwned + Clone + Sync + Send + HasUserId{

    pub fn new(options: AuthTokenServiceOptions) -> Result<Self>{
        let sql_connection = SqlConnection::open(format!("{}/auth_token_{}.db", options.data_directory, options.name)).expect("Could not open SQLite database");
        let sql_connection = Arc::new(Mutex::new(sql_connection));

        if options.drop_table_on_start {
            let connection = sql_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to drop table"))?;
            connection.execute(&DROP, [])?;
        }

        Self::prep_connection(sql_connection.clone())?;

        Ok(Self{
            disposable_token_service: DisposableTokenService::new(options.to_disposable_token_service_options())?,
            timestamp_sorted_list_cache: Arc::new(TimestampSortedListCache::new(options.cache_capacity / 5, options.expiry_seconds, options.max_tokens_per_user as usize)),
            connection: sql_connection,
            options,
        })
    }

    fn prep_connection(sql_connection: Arc<Mutex<SqlConnection>>) -> Result<()>{
        let prep_connection = sql_connection.clone();
        let prep_connection = prep_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to prepare connection"))?;
        // Create the table if it doesn't exist
        let _i = prep_connection.execute(CREATE_TABLE, [])?;
        prep_connection.execute(CREATE_INDEX, [])?;
        prep_connection.execute(CREATE_INDEX_UPDATED, [])?;
        prep_connection.execute(CREATE_INDEX_CREATED, [])?;

        prep_connection.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")?;
        prep_connection.pragma_update(Some(DatabaseName::Main), "synchronous", "normal")?;

        Ok(())
    }

    fn delete_token_from_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let lock = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token from sql"))?;
        let mut statement = lock.prepare_cached(DELETE)?;
        statement.execute([user_id, token_id])?;
        Ok(())
    }

    async fn delete_token_from_sql_async(&self, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        let token_id = token_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_token_from_sql(connection, &user_id, &token_id)
        }).await??;

        Ok(())
    }

    fn add_token_to_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let lock = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to add token to sql"))?;
        let mut statement = lock.prepare_cached(INSERT)?;
        statement.execute([user_id, token_id])?;
        Ok(())
    }

    async fn add_token_to_sql_async(&self, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        let token_id = token_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::add_token_to_sql(connection, &user_id, &token_id)
        }).await??;

        Ok(())
    }

    pub async fn create_token(&self, user_id: &Uuid, token: &T) -> Result<Uuid>{
        // start by creating the token
        let token_id = self.disposable_token_service.create_token(token.clone()).await?;

        self.make_sure_user_exists(user_id).await?;

        let count = self.timestamp_sorted_list_cache.count(user_id).await;

        match count >= self.options.max_tokens_per_user{
            true => {
                for _ in 0..count - self.options.max_tokens_per_user + 1{
                    let oldest_token_id = self.timestamp_sorted_list_cache.pop_oldest(user_id).await;
                    match oldest_token_id {
                        Some(oldest_token_id) => {
                            try_join!(
                                self.disposable_token_service.delete_token(&oldest_token_id),
                                self.delete_token_from_sql_async(user_id, &oldest_token_id)
                            )?;
                        }
                        None => {}
                    }
                }
            }
            false => {}
        }

        try_join!(
            self.add_token_to_sql_async(user_id, &token_id),
            self.timestamp_sorted_list_cache.push_new(user_id, token_id.clone())
        )?;

        Ok(token_id)
    }


    fn get_user_from_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<Option<Vec<(Uuid, i64)>>>{
        let lock = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to check if user is in sql database"))?;
        let mut statement = lock.prepare_cached(SELECT)?;
        let mut rows = statement.query([user_id])?;
        let mut tokens = Vec::new();
        while let Some(row) = rows.next()? {
            let token: Uuid = row.get(0)?;
            let updated: i64 = row.get(1)?;
            tokens.push((token, updated));
        }
        if tokens.len() == 0 {
            return Ok(None);
        }
        Ok(Some(tokens))
    }

    async fn get_user_from_sql_async(&self, user_id: &Uuid) -> Result<Option<Vec<(Uuid, i64)>>>{
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        let res = tokio::task::spawn_blocking(move || {
            Self::get_user_from_sql(connection, &user_id)
        }).await??;

        Ok(res)
    }

    async fn make_sure_user_exists(&self, user_id: &Uuid) -> Result<()>{
        // if the user exists in the timestamp_sorted_list_cache, we are good
        // if the user exists in the database, load them into the timestamp_sorted_list_cache
        // if the user exists in neither, create an empty entry in the timestamp_sorted_list_cache

        match self.timestamp_sorted_list_cache.exists(user_id){
            true => Ok(()),
            false => {
                let tokens = self.get_user_from_sql_async(user_id).await?;
                match tokens {
                    Some(tokens) => {
                        self.timestamp_sorted_list_cache.load(user_id, VecDeque::from(tokens)).await?;
                        Ok(())
                    }
                    None => {
                        self.timestamp_sorted_list_cache.create_empty(user_id).await?;
                        Ok(())
                    }
                }
            }
        }
    }

    fn ping_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let lock = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to ping sql"))?;
        let mut statement = lock.prepare_cached(PING)?;
        statement.execute([user_id, token_id])?;
        Ok(())
    }

    async fn maybe_ping_sql(&self, user_id: &Uuid, token_id: &Uuid) -> Result<()>{
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        let token_id = token_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::ping_sql(connection, &user_id, &token_id)
        }).await??;

        Ok(())
    }

    async fn get_token(&self, token_id: &Uuid) -> Result<Option<T>>{
        match self.disposable_token_service.get_token(token_id).await{
            Ok(Some(token)) => {
                let user_id = token.user_id();
                self.make_sure_user_exists(&user_id).await?;
                self.timestamp_sorted_list_cache.push_new(&user_id, token_id.clone()).await?;
                self.maybe_ping_sql(&user_id, &token_id).await?;
                Ok(Some(token))
            }
            Ok(None) => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    async fn get_token_without_updating(&self, token_id: &Uuid) -> Result<Option<T>>{
        match self.disposable_token_service.get_token(token_id).await{
            Ok(Some(token)) => {
                Ok(Some(token))
            }
            Ok(None) => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    async fn modify_token(&self, token_id: &Uuid, token: T) -> Result<()>{
        self.disposable_token_service.update_token(token_id, token).await
    }

    async fn list_tokens(&self, user_id: &Uuid) -> Result<Vec<Uuid>>{
        self.make_sure_user_exists(&user_id).await?;
        let maybe_token_ids = self.timestamp_sorted_list_cache.get(&user_id).await;
        match maybe_token_ids {
            Some(token_ids) => Ok(token_ids.into_iter().map(|(id, _)| id).collect()),
            None => Ok(Vec::new()),
        }
    }

    fn delete_all_tokens_from_sql(connection: Arc<Mutex<SqlConnection>>, user_id: &Uuid) -> Result<()>{
        let lock = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token from sql"))?;
        let mut statement = lock.prepare_cached(DELETE_ALL)?;
        statement.execute([user_id])?;
        Ok(())
    }

    async fn delete_all_tokens_from_sql_async(&self, user_id: &Uuid) -> Result<()>{
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_all_tokens_from_sql(connection, &user_id)
        }).await??;

        Ok(())
    }

    pub async fn clear_tokens(&self, user_id: &Uuid) -> Result<()>{
        let tokens = self.list_tokens(user_id).await?;
        for token_id in tokens{
            self.disposable_token_service.delete_token(&token_id).await?;
        }
        self.delete_all_tokens_from_sql_async(user_id).await?;
        self.timestamp_sorted_list_cache.clear(user_id).await;

        Ok(())
    }

    /// Clears out the cache for a user without deleting them from the database
    ///
    /// I don't know why we would want to do this outside of a test context:
    ///  forcing the cache to be reloaded from the database the next time it is needed
    /// (to test that the cache reload works)
    pub async fn test_clear_cache(&self, user_id: &Uuid) -> Result<()> {
        self.timestamp_sorted_list_cache.clear(&user_id).await;
        Ok(())
    }

    /// This function should be called periodically to clean up expired tokens
    pub fn clear_expired_tokens(&self) -> Result<()>{
        let lock = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token from sql"))?;
        let mut statement = lock.prepare_cached(DELETE_IDLE)?;
        statement.execute([])?;
        Ok(())
    }

}

impl <T> RequiresBackgroundTick for AuthTokenService<T> where T: Serialize + DeserializeOwned + Clone + Sync + Send + HasUserId{
    fn background_tick(&self) -> Result<()>{
        self.clear_expired_tokens()?;
        self.disposable_token_service.background_tick()?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExampleAuthToken{
    user_id: Uuid,
    name: String,
}

impl HasUserId for ExampleAuthToken{
    fn user_id(&self) -> Uuid{
        self.user_id
    }
}

#[tokio::test]
async fn create_a_bunch_of_auth_tokens(){
    let options = AuthTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "auth".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        max_tokens_per_user: 5,
    };
    let auth_token_service: AuthTokenService<ExampleAuthToken> = AuthTokenService::new(options).unwrap();

    let user_id = Uuid::new_v4();

    let one = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "one".to_string()}).await.unwrap();
    let two = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "two".to_string()}).await.unwrap();
    let three = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "three".to_string()}).await.unwrap();
    let four = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "four".to_string()}).await.unwrap();
    let five = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "five".to_string()}).await.unwrap();
    let six = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "six".to_string()}).await.unwrap();
    let seven = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "seven".to_string()}).await.unwrap();

    let tokens = auth_token_service.list_tokens(&user_id).await.unwrap();
    assert_eq!(tokens.len(), 5);

    assert_eq!(auth_token_service.get_token(&seven).await.unwrap().unwrap().name, "seven");
    assert_eq!(auth_token_service.get_token(&six).await.unwrap().unwrap().name, "six");
    assert_eq!(auth_token_service.get_token(&five).await.unwrap().unwrap().name, "five");
    assert_eq!(auth_token_service.get_token(&four).await.unwrap().unwrap().name, "four");
    assert_eq!(auth_token_service.get_token(&three).await.unwrap().unwrap().name, "three");
    // these two are the oldest, and should have been automatically deleted
    assert!(auth_token_service.get_token(&two).await.unwrap().is_none());
    assert!(auth_token_service.get_token(&one).await.unwrap().is_none());

    auth_token_service.clear_tokens(&user_id).await.unwrap();
    assert_eq!(auth_token_service.list_tokens(&user_id).await.unwrap().len(), 0);
}

#[tokio::test]
async fn clear_cache_and_see_if_it_still_works(){
    let options = AuthTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "auth".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        max_tokens_per_user: 5,
    };
    let auth_token_service: AuthTokenService<ExampleAuthToken> = AuthTokenService::new(options).unwrap();

    let user_id = Uuid::new_v4();

    let one = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "one".to_string()}).await.unwrap();
    let two = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "two".to_string()}).await.unwrap();
    let three = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "three".to_string()}).await.unwrap();
    let four = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "four".to_string()}).await.unwrap();
    let five = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "five".to_string()}).await.unwrap();
    let six = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "six".to_string()}).await.unwrap();
    let seven = auth_token_service.create_token(&user_id, &ExampleAuthToken{user_id: user_id, name: "seven".to_string()}).await.unwrap();

    auth_token_service.test_clear_cache(&user_id).await.unwrap();

    let tokens = auth_token_service.list_tokens(&user_id).await.unwrap();
    assert_eq!(tokens.len(), 5);

    assert_eq!(auth_token_service.get_token(&seven).await.unwrap().unwrap().name, "seven");
    assert_eq!(auth_token_service.get_token(&six).await.unwrap().unwrap().name, "six");
    assert_eq!(auth_token_service.get_token(&five).await.unwrap().unwrap().name, "five");
    assert_eq!(auth_token_service.get_token(&four).await.unwrap().unwrap().name, "four");
    assert_eq!(auth_token_service.get_token(&three).await.unwrap().unwrap().name, "three");
    // these two are the oldest, and should have been automatically deleted
    assert!(auth_token_service.get_token(&two).await.unwrap().is_none());
    assert!(auth_token_service.get_token(&one).await.unwrap().is_none());

    auth_token_service.clear_tokens(&user_id).await.unwrap();
    assert_eq!(auth_token_service.list_tokens(&user_id).await.unwrap().len(), 0);
}