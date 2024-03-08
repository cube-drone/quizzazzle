use std::sync::{Arc, Mutex};
use std::net::IpAddr;

use anyhow::Result;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use rusqlite::{Connection as SqlConnection, params};

use crate::services::create_table::execute_and_eat_already_exists_errors;

#[derive(Clone)]
pub struct UserIpService{
    connection: Arc<Mutex<SqlConnection>>
}

const CREATE_TABLE_IPS: &str = r#"CREATE TABLE IF NOT EXISTS user_ips (
    user_id UUID,
    ip TEXT,
    created INT NOT NULL,
    PRIMARY KEY (user_id, ip)
)"#;
const CREATE_INDEX_CREATED: &str = "CREATE INDEX IF NOT EXISTS user_created ON user_ips (created)";

const INSERT_USER_IP: &str = "INSERT INTO user_ips (user_id, ip, created) VALUES (?1, ?2, unixepoch());";
const CHECK_USER_IP: &str = "SELECT ip FROM user_ips WHERE user_id = ?1 AND ip = ?2;";
const DELETE_IP: &str = "DELETE FROM user_ips WHERE user_id = ?1 AND ip = ?2;";
const DELETE_USER: &str = "DELETE FROM user_ips WHERE user_id = ?1;";

impl UserIpService {
    pub fn new(
        connection: Arc<Mutex<SqlConnection>>,
        drop_table_on_start: bool,
    ) -> Result<Self> {

        if drop_table_on_start {
            let connection = connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to drop table"))?;
            connection.execute("DROP TABLE IF EXISTS user_ips", [])?;
        }

        execute_and_eat_already_exists_errors(connection.clone(), CREATE_TABLE_IPS)?;
        execute_and_eat_already_exists_errors(connection.clone(), CREATE_INDEX_CREATED)?;

        Ok(Self {
            connection
        })
    }

    fn set_user_ip_sql(connection: Arc<Mutex<SqlConnection>>, user_id: Uuid, ip: IpAddr) -> Result<()> {
        let connection = connection.lock().unwrap();
        let mut statement = connection.prepare_cached(INSERT_USER_IP)?;
        statement.execute(params!(&user_id, &ip.to_string()))?;
        Ok(())
    }

    pub async fn set_user_ip(
        &self,
        user_id: Uuid,
        ip: IpAddr
    ) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::set_user_ip_sql(connection, user_id, ip)
        }).await??;

        Ok(())
    }

    pub fn user_has_used_ip(&self, user_id: &Uuid, ip: &IpAddr) -> Result<bool> {
        let connection = self.connection.lock().map_err(|_e| anyhow::anyhow!("Could not get lock to get token"))?;
        let mut statement = connection.prepare_cached(CHECK_USER_IP)?;
        let mut rows = statement.query(params![user_id, ip.to_string()])?;

        match rows.next()?{
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    fn delete_ip_sql(connection: Arc<Mutex<SqlConnection>>, user_id: Uuid, ip: IpAddr) -> Result<()> {
        let connection = connection.lock().unwrap();
        let mut statement = connection.prepare_cached(DELETE_IP)?;
        statement.execute(params!(&user_id, &ip.to_string()))?;
        Ok(())
    }

    pub async fn delete_ip(
        &self,
        user_id: &Uuid,
        ip: IpAddr
    ) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_ip_sql(connection, user_id, ip)
        }).await??;

        Ok(())
    }

    fn delete_user_sql(connection: Arc<Mutex<SqlConnection>>, user_id: Uuid) -> Result<()> {
        let connection = connection.lock().unwrap();
        let mut statement = connection.prepare_cached(DELETE_USER)?;
        statement.execute(params!(&user_id))?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        let connection = self.connection.clone();
        let user_id = user_id.clone();
        tokio::task::spawn_blocking(move || {
            Self::delete_user_sql(connection, user_id)
        }).await??;

        Ok(())
    }


}