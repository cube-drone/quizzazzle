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

const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS tokens (id UUID PRIMARY KEY, value TEXT NOT NULL, created INT NOT NULL, updated INT NOT NULL)";
const CREATE_INDEX_UPDATED: &str = "CREATE INDEX IF NOT EXISTS updated_index ON tokens (updated)";
const CREATE_INDEX_CREATED: &str = "CREATE INDEX IF NOT EXISTS created_index ON tokens (created)";
const INSERT: &str = "INSERT INTO tokens (id, value, created, updated) VALUES (?1, ?2, unixepoch(), unixepoch())";
const UPDATE: &str = "UPDATE tokens SET value = ?2, updated = unixepoch() WHERE id = ?1";
const SELECT: &str = "SELECT value FROM tokens WHERE id = ?1";
const PING: &str = "UPDATE tokens SET updated = CURRENT_TIMESTAMP WHERE id = ?1";
const DELETE: &str = "DELETE FROM tokens WHERE id = ?1";
const DELETE_EXPIRED: &str = "DELETE FROM tokens WHERE created < ?1";
const DELETE_IDLE: &str = "DELETE FROM tokens WHERE updated < ?1";

#[derive(Clone)]
pub struct DisposableTokenServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub get_refreshes_expiry: bool,
    pub probability_of_refresh: f64,
    pub drop_table_on_start: bool,
}

#[derive(Clone)]
pub struct DisposableTokenService<T: 'static> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    cache: Arc<Cache<Uuid, T>>,
    connection: Arc<Mutex<SqlConnection>>,
    options: DisposableTokenServiceOptions,
}

impl<T> DisposableTokenService<T> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    pub fn new(options: DisposableTokenServiceOptions) -> Result<Self>{

        let cache: Cache<Uuid, T>;
        if options.get_refreshes_expiry{
            cache = Cache::builder()
                .max_capacity(options.cache_capacity)
                .time_to_idle(Duration::from_secs(options.expiry_seconds))
                .build();
        }
        else {
            cache = Cache::builder()
                .max_capacity(options.cache_capacity)
                .time_to_live(Duration::from_secs(options.expiry_seconds))
                .build();
        }

        let sql_connection = SqlConnection::open(format!("{}/disposable_token_{}.db", options.data_directory, options.name)).expect("Could not open SQLite database");
        let sql_connection = Arc::new(Mutex::new(sql_connection));

        if options.drop_table_on_start {
            let connection = sql_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to drop table"))?;
            connection.execute("DROP TABLE IF EXISTS tokens", [])?;
        }

        Self::prep_connection(sql_connection.clone())?;

        Ok(Self{
            cache: Arc::new(cache),
            connection: sql_connection,
            options,
        })
    }

    fn prep_connection(sql_connection: Arc<Mutex<SqlConnection>>) -> Result<()>{
        let prep_connection = sql_connection.clone();
        let prep_connection = prep_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to prepare connection"))?;

        // Create the table if it doesn't exist
        let _i = prep_connection.execute(CREATE_TABLE, [])?;
        prep_connection.execute(CREATE_INDEX_CREATED, [])?;
        prep_connection.execute(CREATE_INDEX_UPDATED, [])?;

        // Set the journal mode and synchronous mode: WAL and normal
        // (WAL is write-ahead logging, which is faster and more reliable than the default rollback journal)
        // (normal synchronous mode is the best choice for WAL, and is the best tradeoff between speed and reliability)
        prep_connection.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")?;
        prep_connection.pragma_update(Some(DatabaseName::Main), "synchronous", "normal")?;

        Ok(())
    }

    fn create_sql_token(connection: Arc<Mutex<SqlConnection>>, uuid: &Uuid, value: T) -> Result<()>{
        let serialized_value = serde_json::to_string(&value)?;

        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(INSERT)?;
        let i = statement.execute([&uuid.to_string(), &serialized_value])?;
        if i != 1 {
            return Err(anyhow::anyhow!("Could not insert token"));
        }

        Ok(())
    }

    async fn create_sql_token_async(&self, uuid: &Uuid, value: T) -> Result<()> {
        let connection = self.connection.clone();
        let uuid = uuid.clone();
        let value = value.clone();
        tokio::task::spawn_blocking(move || {
            Self::create_sql_token(connection, &uuid, value)
        }).await??;

        Ok(())
    }

    pub async fn create_token(&self, value: T) -> Result<Uuid>{
        let uuid = Uuid::new_v4();

        let cache_future = self.cache.insert(uuid.clone(), value.clone());
        let sql_future = self.create_sql_token_async(&uuid, value);

        let (_, result) = join!(cache_future, sql_future);
        result?;

        Ok(uuid)
    }

    pub async fn create_token_no_cache(&self, value: T) -> Result<Uuid>{
        let uuid = Uuid::new_v4();

        self.create_sql_token_async(&uuid, value).await?;

        Ok(uuid)
    }

    fn update_sql_token(connection: Arc<Mutex<SqlConnection>>, uuid: &Uuid, value: T) -> Result<()>{
        let serialized_value = serde_json::to_string(&value)?;

        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(UPDATE)?;
        let i = statement.execute([&uuid.to_string(), &serialized_value])?;
        if i != 1 {
            return Err(anyhow::anyhow!("Could not update token"));
        }

        Ok(())
    }

    async fn update_sql_token_async(&self, uuid: &Uuid, value: T) -> Result<()>{
        let connection = self.connection.clone();
        let uuid = uuid.clone();
        let value = value.clone();
        tokio::task::spawn_blocking(move || {
            Self::update_sql_token(connection, &uuid, value)
        }).await??;

        Ok(())
    }

    pub async fn update_token(&self, key: &Uuid, value: T) -> Result<()>{
        let cache_future = self.cache.insert(key.clone(), value.clone());
        let sql_future = self.update_sql_token_async(key, value);

        let (_, result) = join!(cache_future, sql_future);
        result?;

        Ok(())
    }

    fn get_sql_token(connection: Arc<Mutex<SqlConnection>>, key: &Uuid) -> Result<Option<T>>{
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(SELECT)?;
        let mut rows = statement.query([&key.to_string()])?;
        let value = rows.next()?;
        match value{
            Some(v) => {
                // this is a row, and the query JUST asks for value, so we can just get the 0th index
                let serialized_value: String = v.get(0)?;
                let deserialized_value: T = serde_json::from_str(&serialized_value)?;
                Ok(Some(deserialized_value))
            },
            None => Ok(None)
        }
    }

    async fn get_sql_token_async(&self, key: &Uuid) -> Result<Option<T>>{
        let connection = self.connection.clone();
        let key = key.clone();
        tokio::task::spawn_blocking(move || {
            Self::get_sql_token(connection, &key)
        }).await?
    }

    async fn get_and_cache_token(&self, key: &Uuid) -> Result<Option<T>>{
        let value = self.cache.get(key).await;
        match value{
            Some(v) => Ok(Some(v)),
            None => {
                let token = self.get_sql_token_async(key).await;
                match token{
                    Ok(t) => {
                        match t{
                            Some(v) => {
                                self.cache.insert(key.clone(), v.clone()).await;
                                Ok(Some(v))
                            },
                            None => Ok(None)
                        }
                    },
                    Err(e) => {
                        println!("Error getting token: {:?}", e);
                        Ok(None)
                    }
                }
            }
        }
    }

    fn get_sql_tokens(connection: Arc<Mutex<SqlConnection>>, keys: &Vec<Uuid>) -> Result<Vec<(Uuid, T)>>{
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut response: Vec<(Uuid, T)> = Vec::with_capacity(keys.len());
        for key in keys.iter(){
            let mut statement = connection.prepare_cached(SELECT)?;
            let mut rows = statement.query([&key.to_string()])?;
            let value = rows.next()?;
            match value{
                Some(v) => {
                    // this is a row, and the query JUST asks for value, so we can just get the 0th index
                    let serialized_value: String = v.get(0)?;
                    let deserialized_value: T = serde_json::from_str(&serialized_value)?;

                    response.push((key.clone(), deserialized_value));
                },
                None => {}
            }
        }
        Ok(response)
    }

    async fn get_sql_tokens_async(&self, keys: &Vec<Uuid>) -> Result<Vec<(Uuid,T)>>{
        let connection = self.connection.clone();
        let keys = keys.clone();
        tokio::task::spawn_blocking(move || {
            Self::get_sql_tokens(connection, &keys)
        }).await?
    }

    async fn get_tokens(&self, keys: Vec<Uuid>) -> Result<HashMap<Uuid, Option<T>>>{
        let mut result = HashMap::new();

        let mut remaining_keys: Vec<Uuid> = Vec::new();
        for key in keys{
            match self.cache.get(&key).await{
                Some(value) => {
                    result.insert(key, Some(value));
                    continue;
                },
                None => {
                    result.insert(key, None);
                    remaining_keys.push(key.clone());
                }
            }
        }

        let sql_results = self.get_sql_tokens_async(&remaining_keys).await?;
        for (key, value) in sql_results{
            result.insert(key, Some(value));
        }

        Ok(result)
    }

    fn ping_sql_token(connection: Arc<Mutex<SqlConnection>>, key: &Uuid) -> Result<()>{
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to ping token"))?;
        let mut statement = connection.prepare_cached(PING)?;
        statement.execute([&key.to_string()])?;
        Ok(())
    }

    async fn ping_sql_token_async(&self, key: &Uuid) -> Result<()> {
        if self.options.probability_of_refresh < 1.0 {
            let random_number = rand::random::<f64>();
            if random_number > self.options.probability_of_refresh{
                return Ok(());
            }
        }
        let connection = self.connection.clone();
        let key = key.clone();
        tokio::task::spawn_blocking(move || {
            Self::ping_sql_token(connection, &key)
        }).await??;

        Ok(())
    }

    pub async fn get_token(&self, key: &Uuid) -> Result<Option<T>>{
        let token = self.get_and_cache_token(key).await?;
        match token{
            Some(t) => {
                if self.options.get_refreshes_expiry{
                    self.ping_sql_token_async(key).await?;
                }
                Ok(Some(t))
            }
            None => Ok(None)
        }
    }


    fn delete_sql_token(connection: Arc<Mutex<SqlConnection>>, key: &Uuid) -> Result<()>{
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token"))?;
        let mut statement = connection.prepare_cached(DELETE)?;
        statement.execute([&key.to_string()])?;
        Ok(())
    }

    async fn delete_sql_token_async(&self, key: &Uuid) -> Result<()>{
        let connection = self.connection.clone();
        let key = key.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_sql_token(connection, &key)
        }).await??;
        Ok(())
    }

    pub async fn delete_token(&self, key: &Uuid) -> Result<()>{
        let (_, result) = join!(self.cache.remove(key), self.delete_sql_token_async(key));
        result?;

        Ok(())
    }

    pub fn turn_back_time(&self, key: Uuid, seconds: i64) -> Result<()>{
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to turn back time"))?;
        let mut statement = connection.prepare_cached("UPDATE tokens SET created = created - ?1, updated = updated - ?1 WHERE id = ?2")?;
        let i = statement.execute(params![seconds, &key.to_string()])?;
        if i != 1 {
            let mut statement = connection.prepare_cached(SELECT)?;
            let mut rows = statement.query([&key.to_string()])?;
            let value = rows.next()?;
            if value.is_none(){
                return Err(anyhow::anyhow!(format!("Token {} does not exist", key)));
            }
            else{
                return Err(anyhow::anyhow!("Token exists, but something went wrong turning back time"));
            }
        }
        Ok(())
    }

    pub fn delete_expired_tokens(&self) -> Result<()>{
        /*
            This isn't async, because it's not user-facing: it's a background task and will be running on a background thread
         */
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete expired tokens"))?;
        if self.options.get_refreshes_expiry {
            let mut statement = connection.prepare_cached(DELETE_IDLE)?;
            let expiry_timestamp = chrono::Utc::now().timestamp() - self.options.expiry_seconds as i64;
            statement.execute([expiry_timestamp])?;
        }
        else{
            let mut statement = connection.prepare_cached(DELETE_EXPIRED)?;
            let expiry_timestamp = chrono::Utc::now().timestamp() - self.options.expiry_seconds as i64;
            statement.execute([expiry_timestamp])?;
        }
        Ok(())
    }
}

impl <T> RequiresBackgroundTick for DisposableTokenService<T> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    fn background_tick(&self) -> Result<()>{
        self.delete_expired_tokens()?;
        Ok(())
    }
}

#[tokio::test]
async fn test_disposable_token_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test1".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: false,
        probability_of_refresh: 1.0,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let start_time = std::time::Instant::now();

    let token = service.create_token("test".to_string()).await.unwrap();

    for _ in 0..5{
        let value = service.get_token(&token).await.unwrap().unwrap();
        assert_eq!(value, "test".to_string());
    }

    service.update_token(&token, "test2".to_string()).await.unwrap();
    let value = service.get_token(&token).await.unwrap().unwrap();
    assert_eq!(value, "test2".to_string());

    service.delete_token(&token).await.unwrap();
    let value = service.get_token(&token).await.unwrap();
    assert_eq!(value, None);

    let elapsed = start_time.elapsed();
    println!("cache Elapsed: {:?}", elapsed);
}

#[tokio::test]
async fn test_disposable_token_service_no_cache(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test2".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: false,
        probability_of_refresh: 1.0,
    };
    let service = DisposableTokenService::new(options).unwrap();

    let start_time = std::time::Instant::now();

    let token = service.create_token_no_cache("test".to_string()).await.unwrap();

    for _ in 0..5{
        let value = service.get_token(&token).await.unwrap().unwrap();
        assert_eq!(value, "test".to_string());
    }

    service.delete_token(&token).await.unwrap();
    let value = service.get_token(&token).await.unwrap();
    assert_eq!(value, None);

    let elapsed = start_time.elapsed();
    println!("nocache Elapsed: {:?}", elapsed);
}

#[tokio::test]
async fn test_disposable_token_service_speed(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test3".to_string(),
        cache_capacity: 100000,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: false,
        probability_of_refresh: 1.0,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let start_time = std::time::Instant::now();

    let n = 5000;

    for _ in 0..n{
        let token = service.create_token("test".to_string()).await.unwrap();

        for _ in 0..5{
            let value = service.get_token(&token).await.unwrap().unwrap();
            assert_eq!(value, "test".to_string());
        }
    }

    let elapsed = start_time.elapsed();
    println!("{:?} Elapsed per: {:?}µs", n, elapsed.as_micros() as f64 / n as f64);
}

#[tokio::test]
async fn test_idle_token_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test4".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: true,
        probability_of_refresh: 0.5,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let start_time = std::time::Instant::now();

    let token = service.create_token("test".to_string()).await.unwrap();

    for _ in 0..5{
        let value = service.get_token(&token).await.unwrap().unwrap();
        assert_eq!(value, "test".to_string());
    }

    service.delete_token(&token).await.unwrap();
    let value = service.get_token(&token).await.unwrap();
    assert_eq!(value, None);

    let elapsed = start_time.elapsed();
    println!("idle cache Elapsed: {:?}", elapsed);
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct SampleSerializableThing{
    name: String,
    n: i32,
    tags: Vec<String>,
    hash: HashMap<String, String>,
}

#[tokio::test]
async fn test_json_token_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test5".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: false,
        probability_of_refresh: 1.0,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let start_time = std::time::Instant::now();

    let thing = SampleSerializableThing{
        name: "test".to_string(),
        n: 5,
        tags: vec!["a".to_string(), "b".to_string()],
        hash: [("a".to_string(), "b".to_string())].iter().cloned().collect(),
    };

    let token = service.create_token(thing.clone()).await.unwrap();

    for _ in 0..5{
        let value = service.get_token(&token).await.unwrap().unwrap();
        assert_eq!(value, thing);
    }

    let new_thing = SampleSerializableThing{
        name: "test".to_string(),
        n: 6,
        tags: vec!["a".to_string(), "b".to_string()],
        hash: [("a".to_string(), "b".to_string())].iter().cloned().collect(),
    };

    service.update_token(&token, new_thing.clone()).await.unwrap();
    let value = service.get_token(&token).await.unwrap().unwrap();
    assert_eq!(value, new_thing.clone());

    service.delete_token(&token).await.unwrap();
    let value = service.get_token(&token).await.unwrap();
    assert_eq!(value, None);

    let elapsed = start_time.elapsed();
    println!("serialized cache Elapsed: {:?}", elapsed);
}

#[tokio::test]
async fn test_sql_cleanup_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test6".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: false,
        probability_of_refresh: 1.0,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let thing = SampleSerializableThing{
        name: "test".to_string(),
        n: 5,
        tags: vec!["a".to_string(), "b".to_string()],
        hash: [("a".to_string(), "b".to_string())].iter().cloned().collect(),
    };

    let token = service.create_token_no_cache(thing.clone()).await.unwrap();
    let token2 = service.create_token_no_cache(thing.clone()).await.unwrap();
    let token3 = service.create_token_no_cache(thing.clone()).await.unwrap();

    service.turn_back_time(token, 120).unwrap();
    service.turn_back_time(token3, 120).unwrap();

    service.background_tick().unwrap();

    let value = service.get_token(&token).await.unwrap();
    assert_eq!(value, None);
    let value2 = service.get_token(&token2).await.unwrap().unwrap();
    assert_eq!(value2, thing);
    let value3 = service.get_token(&token3).await.unwrap();
    assert_eq!(value3, None);
}

#[tokio::test]
async fn test_get_multiple(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test7".to_string(),
        cache_capacity: 100,
        expiry_seconds: 60,
        drop_table_on_start: true,
        get_refreshes_expiry: true,
        probability_of_refresh: 1.0,
    };

    let service = DisposableTokenService::new(options).unwrap();

    let token = service.create_token("one".to_string()).await.unwrap();
    let token2 = service.create_token_no_cache("two".to_string()).await.unwrap();
    let token3 = service.create_token("three".to_string()).await.unwrap();
    let token4 = service.create_token_no_cache("four".to_string()).await.unwrap();
    let token5 = service.create_token_no_cache("five".to_string()).await.unwrap();
    let fake_token = Uuid::new_v4();

    service.update_token(&token5, "five!".to_string()).await.unwrap();

    let values = service.get_tokens(vec![token, token2, token3, token4, token5, fake_token]).await.unwrap();

    assert_eq!(values.get(&token).unwrap().as_ref().unwrap(), "one");
    assert_eq!(values.get(&token2).unwrap().as_ref().unwrap(), "two");
    assert_eq!(values.get(&token3).unwrap().as_ref().unwrap(), "three");
    assert_eq!(values.get(&token4).unwrap().as_ref().unwrap(), "four");
    assert_eq!(values.get(&token5).unwrap().as_ref().unwrap(), "five!");
    assert!(values.get(&fake_token).unwrap().is_none());

}