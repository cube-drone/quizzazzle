use std::collections::HashMap;
use std::sync::Arc;
use std::env;

use redis::AsyncCommands;
use redis::{SetOptions, SetExpiry};

use anyhow::Result;
use anyhow::anyhow;
use rocket::serde::uuid::Uuid;
use scylla::prepared_statement::PreparedStatement;
//use scylla::frame::value::Timestamp;
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

const ROOT_USER_ID: UserId = UserId(Uuid::from_u128(0));
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
                email text,
                is_verified boolean,
                created_at timestamp,
                updated_at timestamp);
        "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_invite (
                user_id uuid PRIMARY KEY,
                invite_key uuid,
                uses_remaining int,
                created_at timestamp,
                updated_at timestamp);
            "#, &[], ).await?;

    prepared_queries.insert(
        "create_user",
        scylla_session
            .prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, false, ?, ?);")
            .await?,
    );

    prepared_queries.insert(
        "get_user_exists",
        scylla_session
            .prepare("SELECT id FROM ks.user WHERE id = ?;")
            .await?,
    );

    prepared_queries.insert(
        "get_user",
        scylla_session
            .prepare("SELECT id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, created_at, updated_at FROM ks.user WHERE id = ?;")
            .await?,
    );

    /*
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
    */

    Ok(prepared_queries)
}

pub fn hash(password: &str) -> Result<String> {
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(peppered.as_bytes(), &salt).expect("strings should be hashable").to_string();
    Ok(hashed_password)
}

#[derive(Copy, Clone)]
pub struct InviteCode(Uuid);
impl InviteCode {
    pub fn new() -> Self {
        InviteCode(Uuid::new_v4())
    }
    pub fn from_uuid(invite_code: Uuid) -> Self {
        InviteCode(invite_code)
    }
    pub fn from_string(invite_code: &str) -> Result<Self> {
        Ok(InviteCode(Uuid::parse_str(invite_code)?))
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct UserId(Uuid);
impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }
    pub fn from_uuid(user_id: Uuid) -> Self {
        UserId(user_id)
    }
    pub fn from_string(user_id: &str) -> Result<Self> {
        Ok(UserId(Uuid::parse_str(user_id)?))
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct SessionToken(Uuid);
impl SessionToken {
    pub fn new() -> Self {
        SessionToken(Uuid::new_v4())
    }
    pub fn from_uuid(session_token: Uuid) -> Self {
        SessionToken(session_token)
    }
    pub fn from_string(session_token: &str) -> Result<Self> {
        Ok(SessionToken(Uuid::parse_str(session_token)?))
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

pub struct UserCreate<'r>{
    pub user_id: UserId,
    pub parent_id: UserId,
    pub display_name: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}

pub struct UserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub email: String,
}

impl Services {
    pub async fn get_invite_code_source(
        &self,
        invite_code: &InviteCode,
    ) -> Result<UserId> {
        if invite_code.to_uuid() == ROOT_USER_ID.to_uuid(){
            return Err(anyhow!("Invalid invite code"));
        }
        Ok(ROOT_USER_ID)
    }

    pub async fn exhaust_invite_code(
        &self,
        _invite_code: &InviteCode,
    ) -> Result<()> {
        // the invite code can only be used once
        // so we'll just delete it
        Ok(())
    }

    pub async fn get_user_exists(
        &self,
        user_id: &UserId,
    ) -> Result<bool> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user")
                    .expect("Query missing!"),
                (user_id.0,),
            )
            .await?;

        if let Some(rows) = result.rows {
            if rows.len() > 0 {
                return Ok(true);
            }
            else{
                return Ok(false);
            }
        }
        else{
            return Ok(false);
        }
    }

    pub async fn create_root_user(&self) -> Result<()>{
        // don't create a root user if one already exists
        if self.get_user_exists(&ROOT_USER_ID).await? {
            return Ok(());
        }

        let user_id = ROOT_USER_ID.0;
        let display_name = "root";
        let email = "root@gooble.email";
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
                //.prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, false, ?, ?);")
                (user_id, display_name, parent_id, hashed_password, email, DEFAULT_THUMBNAIL_URL, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
            )
            .await?;

        Ok(())
    }

    /*
    pub async fn send_verification_email(
        &self,
        user_id: &Uuid,
    ) -> Result<()> {
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let email_verification_token = Uuid::new_v4().to_string();
        redis_connection.set(&format!("user:{}:email_verification_token", user_id), email_verification_token, "EX", 86400 * 3).await?;

        Ok(())
    }
    */

    pub async fn create_user(
        &self,
        user_create: UserCreate<'_>,
    ) -> Result<SessionToken> {
        let hashed_password = hash(&user_create.password)?;

        if self.get_user_exists(&user_create.user_id).await? {
            return Err(anyhow!("User somehow already exists! Wow, UUIDs are not as unique as I thought!"));
        }
        if !self.get_user_exists(&user_create.parent_id).await? {
            return Err(anyhow!("Parent user does not exist!"));
        }
        // test the email doesn't already exist (if it exists on an unverified user, we'll just destroy them)
        // TODO

        // the core user record!
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("create_user")
                    .expect("Query missing!"),
                //.prepare("INSERT INTO ks.user (id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, false, ?, ?);")
                (
                    user_create.user_id.0,
                    user_create.display_name,
                    user_create.parent_id.0,
                    hashed_password,
                    user_create.email,
                    DEFAULT_THUMBNAIL_URL,
                    Utc::now().timestamp_millis(),
                    Utc::now().timestamp_millis()
                ),
            )
            .await?;

        //send_verification_email(&self, &user_id).await?;

        let session_token = self.create_session_token(&user_create.user_id).await?;

        Ok(SessionToken(session_token))
    }

    pub async fn create_session_token(&self, user_id: &UserId) -> Result<Uuid>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let session_token = Uuid::new_v4();
        let options = SetOptions::default().with_expiration(SetExpiry::EX(86400 * 3));
        redis_connection.set_options(&format!("session_token:{}", session_token.to_string()), user_id.0.to_string(), options).await?;

        Ok(session_token)
    }

    pub async fn get_user_session(&self, user_id: &UserId) -> Result<UserSession>{
        /*
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user")
                    .expect("Query missing!"),
                (user_id,),
            )
            .await?;

        if let Some(rows) = result.rows {
            if rows.len() > 0 {
                let row = rows.get(0).unwrap();
                let display_name: String = row.get_r_by_name("display_name")?;
                let thumbnail_url: String = row.get_r_by_name("thumbnail_url")?;
                let is_verified: bool = row.get_r_by_name("is_verified")?;
                let email: String = row.get_r_by_name("email")?;

                return Ok(UserSession{
                    user_id: *user_id,
                    display_name,
                    thumbnail_url,
                    is_verified,
                    email,
                });
            }
            else{
                return Err(anyhow!("User does not exist!"));
            }
        }
        else{
            return Err(anyhow!("User does not exist!"));
        }
        */
        return Ok(UserSession{
            user_id: *user_id,
            display_name: "root".to_string(),
            thumbnail_url: DEFAULT_THUMBNAIL_URL.to_string(),
            is_verified: true,
            email: "fake@fake.fake".to_string(),
        });
    }

    pub async fn get_user_from_session_token(&self, session_token: &SessionToken) -> Result<UserSession>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let user_id: String = redis_connection.get(&format!("session_token:{}", session_token.0)).await?;
        if user_id == "" {
            return Err(anyhow!("Session token does not exist!"));
        }

        let user_id = UserId(Uuid::parse_str(&user_id)?);

        let user_session = self.get_user_session(&user_id).await?;

        return Ok(user_session);
    }

}