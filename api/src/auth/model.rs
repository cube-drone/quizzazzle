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

    pub async fn _create_user(
        &self,
        display_name: &str,
        parent_id: Uuid,
        password: &str,
        thumbnail_url: &str,
    ) -> Result<Uuid> {
        let user_id = Uuid::new_v4();
        let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
        let salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let hashed_password = argon2.hash_password(peppered.as_bytes(), &salt).expect("passwords should be hashable").to_string();
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("create_user")
                    .expect("Query missing!"),
                (user_id, display_name, parent_id, hashed_password, thumbnail_url, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
            )
            .await?;
        Ok(user_id)
    }

}