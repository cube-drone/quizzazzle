use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use anyhow::Result;
use rocket::serde::uuid::Uuid;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, Result as SqlResult, DatabaseName};

use serde::Serialize;
use serde::de::DeserializeOwned;


const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS tokens (id UUID PRIMARY KEY, value TEXT NOT NULL, created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)";
const INSERT: &str = "INSERT INTO tokens (id, value) VALUES (?1, ?2)";
const SELECT: &str = "SELECT value FROM tokens WHERE id = ?1";
const DELETE: &str = "DELETE FROM tokens WHERE id = ?1";
const DELETE_EXPIRED: &str = "DELETE FROM tokens WHERE created < ?1";

pub struct DisposableTokenServiceOptions{
    pub data_directory: String,
    pub name: String,
    pub cache_capacity: u64,
    pub expiry_seconds: u64,
    pub drop_table_on_start: bool,
}

pub struct DisposableTokenService<T: 'static> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    cache: Cache<Uuid, T>,
    connection: Arc<Mutex<SqlConnection>>,
    options: DisposableTokenServiceOptions,
}


impl<T> DisposableTokenService<T> where T: Serialize + DeserializeOwned + Clone + Sync + Send{
    pub fn new(options: DisposableTokenServiceOptions) -> Result<Self>{
        let cache: Cache<Uuid, T> = Cache::builder()
            .max_capacity(options.cache_capacity)
            .time_to_live(Duration::from_secs(options.expiry_seconds))
            .build();

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

        // Prepare the statement for later use
        // the connection will retrieve this cached statement any time we use prepare_cached to get the same sql statement
        prep_connection.prepare_cached(INSERT)?;
        prep_connection.prepare_cached(SELECT)?;
        prep_connection.prepare_cached(DELETE)?;
        Ok(())
    }

    fn create_sql_token(&self, uuid: &Uuid, value: T) -> Result<()>{
        let serialized_value = serde_json::to_string(&value)?;

        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to create token"))?;
        let mut statement = connection.prepare_cached(INSERT)?;
        statement.execute([&uuid.to_string(), &serialized_value])?;

        Ok(())
    }

    pub async fn create_token(&self, value: T) -> Result<Uuid>{
        let uuid = Uuid::new_v4();
        self.cache.insert(uuid.clone(), value.clone()).await;

        self.create_sql_token(&uuid, value)?;

        Ok(uuid)
    }


    fn get_sql_token(&self, key: &Uuid) -> Result<Option<T>>{
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(SELECT)?;
        let mut rows = statement.query([&key.to_string()])?;
        let value: SqlResult<String> = rows.next()?.unwrap().get(0);
        match value{
            Ok(v) => {
                let deserialized_value: T = serde_json::from_str(&v)?;
                Ok(Some(deserialized_value))
            },
            Err(_) => Ok(None)
        }
    }

    pub async fn get_token(&self, key: &Uuid) -> Result<Option<T>>{
        let value = self.cache.get(key).await;
        match value{
            Some(v) => Ok(Some(v)),
            None => {
                self.get_sql_token(key)
            }
        }
    }

    fn delete_sql_token(&self, key: &Uuid) -> Result<()>{
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete token"))?;
        let mut statement = connection.prepare_cached(DELETE)?;
        statement.execute([&key.to_string()])?;
        Ok(())
    }

    pub async fn delete_token(&self, key: &Uuid) -> Result<()>{
        self.cache.remove(key).await;
        self.delete_sql_token(key)?;
        Ok(())
    }

    pub fn delete_expired_tokens(&self) -> Result<()>{
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to delete expired tokens"))?;
        let mut statement = connection.prepare_cached(DELETE_EXPIRED)?;
        let expiry_timestamp = chrono::Utc::now().timestamp() - self.options.expiry_seconds as i64;
        statement.execute([expiry_timestamp])?;
        Ok(())
    }
}