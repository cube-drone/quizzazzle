use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use anyhow::Result;
use futures::join;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, DatabaseName};

use serde::Serialize;
use serde::de::DeserializeOwned;


const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS tokens (id UUID PRIMARY KEY, value TEXT NOT NULL, created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)";
const INSERT: &str = "INSERT INTO tokens (id, value) VALUES (?1, ?2)";
const SELECT: &str = "SELECT value FROM tokens WHERE id = ?1";
const PING: &str = "UPDATE tokens SET updated = CURRENT_TIMESTAMP WHERE id = ?1";
const DELETE: &str = "DELETE FROM tokens WHERE id = ?1";
const DELETE_EXPIRED: &str = "DELETE FROM tokens WHERE created < ?1";
const DELETE_IDLE: &str = "DELETE FROM tokens WHERE updated < ?1";

pub struct DisposableTokenServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub get_refreshes_expiry: bool,
    pub probability_of_refresh: f64,
    pub drop_table_on_start: bool,
}

pub struct DisposableTokenService<T: 'static> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    cache: Cache<Uuid, T>,
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
            cache,
            connection: sql_connection,
            options,
        })
    }

    fn prep_connection(sql_connection: Arc<Mutex<SqlConnection>>) -> Result<()>{
        let prep_connection = sql_connection.clone();
        let prep_connection = prep_connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to prepare connection"))?;
        // Create the table if it doesn't exist
        prep_connection.execute(CREATE_TABLE, [])?;

        prep_connection.pragma_update(Some(DatabaseName::Main), "journal_mode", "WAL")?;
        prep_connection.pragma_update(Some(DatabaseName::Main), "synchronous", "off")?;

        // Prepare the statement for later use
        // the connection will retrieve this cached statement any time we use prepare_cached to get the same sql statement
        prep_connection.prepare_cached(INSERT)?;
        prep_connection.prepare_cached(PING)?;
        prep_connection.prepare_cached(SELECT)?;
        prep_connection.prepare_cached(DELETE)?;
        prep_connection.prepare_cached(DELETE_EXPIRED)?;
        prep_connection.prepare_cached(DELETE_IDLE)?;
        Ok(())
    }

    fn create_sql_token(connection: Arc<Mutex<SqlConnection>>, uuid: &Uuid, value: T) -> Result<()>{
        let serialized_value = serde_json::to_string(&value)?;

        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(INSERT)?;
        statement.execute([&uuid.to_string(), &serialized_value])?;

        Ok(())
    }

    pub async fn create_sql_token_async(&self, uuid: &Uuid, value: T) {
        let connection = self.connection.clone();
        let uuid = uuid.clone();
        let value = value.clone();
        match tokio::task::spawn_blocking(move || {
            let result = Self::create_sql_token(connection, &uuid, value);
            match result{
                Ok(_) => {},
                Err(e) => {
                    println!("Error creating token: {:?}", e);
                }
            }
        }).await{
            Ok(_) => {},
            Err(e) => {
                println!("Error creating token: {:?}", e);
            }
        }
    }

    pub async fn create_token(&self, value: T) -> Result<Uuid>{
        let uuid = Uuid::new_v4();

        let cache_future = self.cache.insert(uuid.clone(), value.clone());
        let sql_future = self.create_sql_token_async(&uuid, value);

        join!(cache_future, sql_future);

        Ok(uuid)
    }

    pub async fn create_token_no_cache(&self, value: T) -> Result<Uuid>{
        let uuid = Uuid::new_v4();

        self.create_sql_token_async(&uuid, value).await;

        Ok(uuid)
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

    #[allow(unused_assignments)]
    async fn get_sql_token_async(&self, key: &Uuid) -> Result<Option<T>>{
        let connection = self.connection.clone();
        let key = key.clone();
        let mut res: Result<Option<T>> = Ok(None);
        res = tokio::task::spawn_blocking(move || {
            res = Self::get_sql_token(connection, &key);
            res
        }).await?;

        res
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

    fn ping_sql_token(connection: Arc<Mutex<SqlConnection>>, key: &Uuid) -> Result<()>{
        let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to ping token"))?;
        let mut statement = connection.prepare_cached(PING)?;
        statement.execute([&key.to_string()])?;
        Ok(())
    }

    async fn ping_sql_token_async(&self, key: &Uuid){
        if self.options.probability_of_refresh < 1.0 {
            let random_number = rand::random::<f64>();
            if random_number > self.options.probability_of_refresh{
                return;
            }
        }
        let connection = self.connection.clone();
        let key = key.clone();
        match tokio::task::spawn_blocking(move || {
            let result = Self::ping_sql_token(connection, &key);
            match result{
                Ok(_) => {},
                Err(e) => {
                    println!("Error pinging token: {:?}", e);
                }
            }
        }).await{
            Ok(_) => {},
            Err(e) => {
                println!("Error pinging token: {:?}", e);
            }
        }
    }

    pub async fn get_token(&self, key: &Uuid) -> Result<Option<T>>{
        let token = self.get_and_cache_token(key).await?;
        match token{
            Some(t) => {
                if self.options.get_refreshes_expiry{
                    self.ping_sql_token_async(key).await;
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

    async fn delete_sql_token_async(&self, key: &Uuid){
        let connection = self.connection.clone();
        let key = key.clone();
        match tokio::task::spawn_blocking(move || {
            let result = Self::delete_sql_token(connection, &key);
            match result{
                Ok(_) => {},
                Err(e) => {
                    println!("Error deleting token: {:?}", e);
                }
            }
        }).await{
            Ok(_) => {},
            Err(e) => {
                println!("Error deleting token: {:?}", e);
            }
        }
    }

    pub async fn delete_token(&self, key: &Uuid) -> Result<()>{
        join!(self.cache.remove(key), self.delete_sql_token_async(key));

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

#[tokio::test]
async fn test_disposable_token_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test".to_string(),
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
        name: "test".to_string(),
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
        name: "test".to_string(),
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

        service.delete_token(&token).await.unwrap();
        let value = service.get_token(&token).await.unwrap();
        assert_eq!(value, None);
    }

    let elapsed = start_time.elapsed();
    println!("{:?} Elapsed per: {:?}µs", n, elapsed.as_micros() as f64 / n as f64);
}

#[tokio::test]
async fn test_idle_token_service(){
    let options = DisposableTokenServiceOptions{
        data_directory: "./test_data".to_string(),
        name: "test".to_string(),
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
    println!("cache Elapsed: {:?}", elapsed);
}