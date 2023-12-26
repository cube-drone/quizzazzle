use std::collections::HashMap;
use std::sync::Arc;
use std::env;

use anyhow::Result;
use anyhow::anyhow;
use rocket::serde::uuid::Uuid;
use scylla::prepared_statement::PreparedStatement;
use scylla::frame::value::Timestamp;
use scylla::Session;
use chrono::{Utc};

use ::argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::ScyllaService;
use crate::Services;

const ROOT_USER_ID: Uuid = Uuid::from_u128(0);
const DEFAULT_THUMBNAIL_URL: &str = "/static/chismas.png";

pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut prepared_queries = HashMap::new();
    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user (
                id uuid PRIMARY KEY,
                display_name text,
                parent_id uuid,
                hashed_password text,
                thumbnail_url text,
                is_verified bool,
                created_at timestamp,
                updated_at timestamp)
        "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_invite (
                user_id uuid PRIMARY KEY,
                invite_key uuid,
                uses_remaining int,
                created_at timestamp,
                updated_at timestamp)
            "#, &[], ).await?;

    prepared_queries.insert(
        "create_user",
        scylla_session
            .prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, false, ?, ?);")
            .await?,
    );

    prepared_queries.insert(
        "get_user_exists",
        scylla_session
            .prepare("SELECT id FROM ks.user WHERE id = ?")
            .await?,
    );

    prepared_queries.insert(
        "create_user_invite",
        scylla_session
            .prepare("INSERT INTO ks.user_invite (user_id, invite_key, uses_remaining, created_at, updated_at) VALUES (?, ?, ?, ?, ?);")
            .await?,
    );

    prepared_queries.insert(
        "update_user_password",
        scylla_session
            .prepare("UPDATE ks.user USING TTL 0 SET hashed_password = ? WHERE id = ?;")
            .await?,
    );

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_parents (
                user_id uuid PRIMARY KEY,
                parents list<uuid>)
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_children (
                "user_id uuid PRIMARY KEY,
                "children list<uuid>)
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_ancestors (
                user_id uuid PRIMARY KEY, " +
                ancestors list<uuid>)
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_ips (
                user_id uuid PRIMARY KEY,
                ips list<uuid>)
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_email (
                user_id uuid,
                email text,
                email_domain text,
                primary_email boolean,
                verified boolean,
                verification_token text,
                created_at timestamp,
                updated_at timestamp,
                PRIMARY KEY (user_id, email))
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_user (
                email text PRIMARY KEY,
                user_id uuid)
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_domain (
                email_domain text,
                user_id uuid, " +
                PRIMARY KEY (email_domain, user_id))
            "#, &[], ).await?;

    Ok(prepared_queries)
}

pub fn hash(password: &str) -> Result<String> {
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(peppered.as_bytes(), &salt)?.to_string();
    Ok(hashed_password)
}

impl Services {
    pub async fn get_invite_code_source(
        &self,
        invite_code: &str,
    ) -> Result<Uuid> {
        if invite_code == "invalid" {
            return Err(anyhow!("Invalid invite code"));
        }
        Ok(ROOT_USER_ID)
    }

    pub async fn exhaust_invite_code(
        &self,
        invite_code: &str,
    ) -> Result<()> {
        // the invite code can only be used once
        // so we'll just delete it
        Ok(())
    }

    pub async fn get_user_exists(
        &self,
        user_id: &Uuid,
    ) -> Result<bool> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user_exists")
                    .expect("Query missing!"),
                (user_id,),
            )
            .await?;

        Ok(result.rows.len() > 0)
    }

    pub async fn create_root_user(&self) -> Result<()>{
        // don't create a root user if one already exists
        if self.get_user_exists(&ROOT_USER_ID).await? {
            return Ok(());
        }

        let user_id = ROOT_USER_ID;
        let display_name = "root";
        let parent_id = "";
        let root_auth_password = env::var("GROOVELET_ROOT_AUTH_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let hashed_password = hash(&root_auth_password)?;

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("create_user")
                    .expect("Query missing!"),
                //.prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, false, ?, ?);")
                (user_id, display_name, parent_id, hashed_password, DEFAULT_THUMBNAIL_URL, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
            )
            .await?;

        Ok(())
    }

    pub async fn create_user(
        &self,
        display_name: &str,
        parent_id: Uuid,
        password: &str,
    ) -> Result<Uuid> {
        let user_id = Uuid::new_v4();
        let hashed_password = hash(&password)?;

        if self.get_user_exists(&user_id).await? {
            return Err(anyhow!("User somehow already exists! Wow, UUIDs are not as unique as I thought!"));
        }
        if !self.get_user_exists(&parent_id).await? {
            return Err(anyhow!("Parent user does not exist!"));
        }

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("create_user")
                    .expect("Query missing!"),
                //.prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, false, ?, ?);")
                (user_id, display_name, parent_id, hashed_password, DEFAULT_THUMBNAIL_URL, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
            )
            .await?;

        Ok(user_id)
    }

}