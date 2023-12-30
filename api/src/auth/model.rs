use std::collections::HashMap;
use std::sync::Arc;
use std::env;

use serde::{Deserialize, Serialize};

use redis::AsyncCommands;

use anyhow::Result;
use anyhow::anyhow;
use rocket::serde::uuid::Uuid;
use scylla::prepared_statement::PreparedStatement;
use scylla::macros::FromRow;
//use scylla::frame::value::Timestamp;
use scylla::Session;
use chrono::{Utc, Duration};

use ::argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::email::EmailAddress;
use crate::Services;

const ROOT_USER_ID: UserId = UserId(Uuid::from_u128(0));
const DEFAULT_THUMBNAIL_URL: &str = "/static/chismas.png";

const USER_SESSION_TIMEOUT_SECONDS: usize = 86400 * 14; // two weeks
const EMAIL_VERIFICATION_TIMEOUT_SECONDS: usize = 86400 * 3; // three days
const USER_MAX_SESSION_COUNT: usize = 8; // how many sessions can a single user have active?

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
                created_at timestamp );
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

    prepared_queries.insert(
        "verify_user_email",
        scylla_session
            .prepare("UPDATE ks.user SET is_verified = true WHERE id = ?;")
            .await?,
    );

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_parents (
                user_id uuid PRIMARY KEY,
                parents list<uuid> );
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_children (
                user_id uuid,
                child_id uuid,
                PRIMARY KEY (user_id, child_id));
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_descendants (
                user_id uuid,
                descendant_id uuid,
                PRIMARY KEY (user_id, descendant_id));
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_ips (
                user_id uuid,
                ip inet,
                PRIMARY KEY(user_id, ip));
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_user (
                email text PRIMARY KEY,
                user_id uuid)
            "#, &[], ).await?;

    prepared_queries.insert(
        "get_email_user",
        scylla_session
            .prepare("SELECT user_id FROM ks.email_user WHERE email = ?;")
            .await?,
    );

    prepared_queries.insert(
        "set_email_user",
        scylla_session
            .prepare("INSERT INTO ks.email_user (email, user_id) VALUES (?, ?);")
            .await?,
    );

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_domain (
                email_domain text,
                user_id uuid,
                PRIMARY KEY (email_domain, user_id))
            "#, &[], ).await?;

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

    */

    Ok(prepared_queries)
}

pub fn password_hash(password: &str) -> Result<String> {
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(peppered.as_bytes(), &salt).expect("strings should be hashable").to_string();
    Ok(hashed_password)
}

pub fn password_test(password: &str, hashed_password: &str) -> Result<bool> {
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hashed_password).unwrap();
    let is_valid = argon2
        .verify_password(peppered.as_bytes(), &password_hash)
        .is_ok();
    Ok(is_valid)
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreate<'r>{
    pub user_id: UserId,
    pub parent_id: UserId,
    pub display_name: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
}

impl UserSession {
    pub fn to_verified_user_session(&self) -> VerifiedUserSession {
        VerifiedUserSession {
            user_id: self.user_id,
            display_name: self.display_name.clone(),
            thumbnail_url: self.thumbnail_url.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifiedUserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
}

#[derive(FromRow)]
pub struct UserDatabaseRaw {
    pub id: Uuid,
    pub display_name: String,
    pub parent_id: Option<Uuid>,
    pub hashed_password: String,
    pub email: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub created_at: Duration,
    pub updated_at: Duration,
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

    pub async fn generate_invite_code(
        &self) -> Result<InviteCode> {
        // for testing, generate a new invite code from the root user
        Ok(InviteCode::new())
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

    pub async fn get_user(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserDatabaseRaw>> {
        println!("get_user: {}", user_id.to_string());
        Ok(self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
            )
            .await?
            .maybe_first_row_typed::<UserDatabaseRaw>()?)
    }

    pub async fn get_user_email(
        &self,
        email: &str,
    ) -> Result<Option<UserDatabaseRaw>> {
        println!("get_user_email: {}", email);
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_email_user")
                    .expect("Query missing!"),
                (email,),
            )
            .await?;

        if let Some(rows) = result.rows {
            if rows.len() > 0 {
                let row = rows.get(0).unwrap();
                let user_id: Uuid = row.columns[0].as_ref().unwrap().as_uuid().unwrap();
                let user_id = UserId(user_id);
                return self.get_user(&user_id).await;
            }
            else{
                return Ok(None);
            }
        }
        else{
            return Ok(None);
        }
    }

    pub async fn create_root_user(&self) -> Result<()>{
        // don't create a root user if one already exists
        if self.get_user_exists(&ROOT_USER_ID).await? {
            return Ok(());
        }

        let user_id = ROOT_USER_ID.to_uuid();
        let display_name = "root";
        let email = "root@gooble.email";
        let parent_id = "";
        let root_auth_password = env::var("GROOVELET_ROOT_AUTH_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let hashed_password = password_hash(&root_auth_password)?;


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

        // email -> user
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_email_user")
                    .expect("query missing!"),
                (email, user_id,),
            )
            .await?;

        Ok(())
    }

    pub async fn create_user(
        &self,
        user_create: UserCreate<'_>,
    ) -> Result<SessionToken> {
        let hashed_password = password_hash(&user_create.password)?;

        if self.get_user_exists(&user_create.user_id).await? {
            return Err(anyhow!("User somehow already exists! Wow, UUIDs are not as unique as I thought!"));
        }
        if !self.get_user_exists(&user_create.parent_id).await? {
            return Err(anyhow!("Parent user does not exist!"));
        }
        let email_user = self.get_user_email(&user_create.email).await?;
        if let Some(email_user) = email_user {
            if email_user.is_verified{
                return Err(anyhow!("Email already exists!"));
            }
            else{
                // TODO: delete the unverified user
                // and just create a new one, now
                return Err(anyhow!("Email already exists, but is not verified!"));
            }
        }

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

        // email -> user
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_email_user")
                    .expect("query missing!"),
                (user_create.email, user_create.user_id.0,),
            )
            .await?;

        self.send_verification_email( &user_create.user_id.0, &user_create.email ).await?;

        let user_session = UserSession{
            user_id: user_create.user_id,
            display_name: user_create.display_name.to_string(),
            thumbnail_url: DEFAULT_THUMBNAIL_URL.to_string(),
            is_verified: false,
        };

        let session_token = self.create_session_token(&user_session).await?;

        Ok(session_token)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<SessionToken> {
        let email_user = self.get_user_email(&email).await?;
        if let Some(email_user) = email_user {
            if password_test(&password, &email_user.hashed_password)? {
                let user_id: UserId = UserId::from_uuid(email_user.id);
                let user_session: UserSession = UserSession{
                    user_id: user_id,
                    display_name: email_user.display_name,
                    thumbnail_url: email_user.thumbnail_url,
                    is_verified: email_user.is_verified,
                };

                let session_token = self.create_session_token(&user_session).await?;
                return Ok(session_token);
            }
        }
        Err(anyhow!("Invalid email or password!"))
    }


    pub async fn send_verification_email(
        &self,
        user_id: &Uuid,
        email_address: &str,
    ) -> Result<()> {
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let email_verification_token = Uuid::new_v4().to_string();
        let key = format!("email_verification_token:${}", email_verification_token);

        redis_connection.set_ex(&key, user_id.to_string(), EMAIL_VERIFICATION_TIMEOUT_SECONDS).await?;

        let public_address = self.config_get_public_address();

        let email_verification_link = format!("{}/auth/verify_email?token={}", public_address, email_verification_token);

        self.email.send_verification_email(&EmailAddress::new(email_address.to_string())?, &email_verification_link).await?;

        if ! self.is_production {
            let last_email_sent_key = format!("last_email_sent:${}", email_address);
            redis_connection.set(&last_email_sent_key, email_verification_link).await?;
        }

        Ok(())
    }

    pub async fn test_get_last_email(&self, email_address: &str) -> Result<String> {
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let last_email_sent_key = format!("last_email_sent:${}", email_address);
        let last_email_sent: String = redis_connection.get(&last_email_sent_key).await?;
        Ok(last_email_sent)
    }

    pub async fn verify_email(
        &self,
        email_verification_token: &Uuid,
    ) -> Result<UserId> {
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let verification_token_key = format!("email_verification_token:${}", email_verification_token.to_string());
        let user_id: String = redis_connection.get(&verification_token_key).await?;
        let user_id = Uuid::parse_str(&user_id)?;
        let user_id = UserId(user_id);

        if ! self.get_user_exists(&user_id).await? {
            return Err(anyhow!("User does not exist!"));
        }

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("verify_user_email")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
            )
            .await?;

        self.verify_all_sessions(&user_id).await?;

        redis_connection.unlink(&verification_token_key).await?;

        Ok(user_id)
    }

/*


                                                                         iiii
                                                                        i::::i
                                                                         iiii

    ssssssssss       eeeeeeeeeeee        ssssssssss       ssssssssss   iiiiiii    ooooooooooo   nnnn  nnnnnnnn
  ss::::::::::s    ee::::::::::::ee    ss::::::::::s    ss::::::::::s  i:::::i  oo:::::::::::oo n:::nn::::::::nn
ss:::::::::::::s  e::::::eeeee:::::eess:::::::::::::s ss:::::::::::::s  i::::i o:::::::::::::::on::::::::::::::nn
s::::::ssss:::::se::::::e     e:::::es::::::ssss:::::ss::::::ssss:::::s i::::i o:::::ooooo:::::onn:::::::::::::::n
 s:::::s  ssssss e:::::::eeeee::::::e s:::::s  ssssss  s:::::s  ssssss  i::::i o::::o     o::::o  n:::::nnnn:::::n
   s::::::s      e:::::::::::::::::e    s::::::s         s::::::s       i::::i o::::o     o::::o  n::::n    n::::n
      s::::::s   e::::::eeeeeeeeeee        s::::::s         s::::::s    i::::i o::::o     o::::o  n::::n    n::::n
ssssss   s:::::s e:::::::e           ssssss   s:::::s ssssss   s:::::s  i::::i o::::o     o::::o  n::::n    n::::n
s:::::ssss::::::se::::::::e          s:::::ssss::::::ss:::::ssss::::::si::::::io:::::ooooo:::::o  n::::n    n::::n
s::::::::::::::s  e::::::::eeeeeeee  s::::::::::::::s s::::::::::::::s i::::::io:::::::::::::::o  n::::n    n::::n
 s:::::::::::ss    ee:::::::::::::e   s:::::::::::ss   s:::::::::::ss  i::::::i oo:::::::::::oo   n::::n    n::::n
  sssssssssss        eeeeeeeeeeeeee    sssssssssss      sssssssssss    iiiiiiii   ooooooooooo     nnnnnn    nnnnnn


*/

    pub async fn create_session_token(&self, user_session: &UserSession) -> Result<SessionToken>{
        /*
            given a user session, create a session token and save it in redis
         */
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let session_token = Uuid::new_v4();
        let session_json = serde_json::to_string(user_session)?;
        let key = format!("session_token:{}", session_token.to_string());
        redis_connection.set_ex(&key, session_json, USER_SESSION_TIMEOUT_SECONDS).await?;

        let user_sessions_key = format!("user_sessions:{}", user_session.user_id.to_string());
        redis_connection.zadd(&user_sessions_key, session_token.to_string(), Utc::now().timestamp_millis()).await?;
        redis_connection.expire(&user_sessions_key, USER_SESSION_TIMEOUT_SECONDS*2).await?;

        let user_sessions_count: usize = redis_connection.zcard(&user_sessions_key).await?;
        //println!("user_sessions_count: {}", user_sessions_count);
        // if the user has more than MAX_SESSIONS sessions, delete the oldest one
        if user_sessions_count > USER_MAX_SESSION_COUNT {
            self.cull_old_sessions(&user_session.user_id).await?;
        }

        Ok(SessionToken(session_token))
    }

    pub async fn cull_old_sessions(&self, user_id: &UserId) -> Result<()>{
        // the user has more than USER_MAX_SESSION_COUNT sessions, delete all but the USER_MAX_SESSION_COUNT most recent
        // it's also fine to cull any that have obviously expired (> USER_SESSION_TIMEOUT_SECONDS old)
        let timestamp_cutoff: i64 = Utc::now().timestamp_millis() - (USER_SESSION_TIMEOUT_SECONDS as i64 * 1000);

        let mut counter: usize = 0;
        for (session_token, timestamp) in self.get_all_sessions(&user_id).await? {
            if timestamp < timestamp_cutoff || counter > USER_MAX_SESSION_COUNT {
                self.delete_session(&session_token, &user_id).await?;
            }
            counter += 1;
        }

        Ok(())
    }

    pub async fn delete_session(&self, session_token: &SessionToken, user_id: &UserId) -> Result<()>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        redis_connection.unlink(&format!("session_token:{}", session_token.to_string())).await?;
        redis_connection.zrem(&format!("user_sessions:{}", user_id.to_string()), session_token.to_string()).await?;

        Ok(())
    }

    pub async fn delete_all_sessions(&self, user_id: &UserId) -> Result<()>{
        for (session_token, _timestamp) in self.get_all_sessions(&user_id).await? {
            self.delete_session(&session_token, &user_id).await?;
        }
        Ok(())
    }

    pub async fn verify_session(&self, session_token: &SessionToken) -> Result<()>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let key = format!("session_token:{}", session_token.to_string());
        let session_json: String = redis_connection.get(&key).await?;

        //println!("verifying session_json: {}", session_json);

        let mut user_session: UserSession = serde_json::from_str(&session_json)?;

        user_session.is_verified = true;

        let session_json = serde_json::to_string(&user_session)?;

        redis_connection.set_ex(&key, session_json, USER_SESSION_TIMEOUT_SECONDS).await?;

        Ok(())
    }

    pub async fn get_all_sessions(&self, user_id: &UserId) -> Result<Vec<(SessionToken, i64)>> {
        let mut redis_connection = self.application_redis.get_async_connection().await?;
        let user_sessions: Vec<(String, i64)> = redis_connection.zrangebyscore_withscores(&format!("user_sessions:{}", user_id.to_string()), "-inf", "+inf").await?;

        let new_user_sessions: Vec<(SessionToken, i64)> = user_sessions.iter().map(|(session_token, timestamp)| (SessionToken::from_string(&session_token).unwrap(), *timestamp)).collect();

        Ok(new_user_sessions)
    }

    pub async fn verify_all_sessions(&self, user_id: &UserId) -> Result<()>{
        /*
            after the user has verified that their email address is valid, we should verify all of their sessions
         */
        for (session_token, _timestamp) in self.get_all_sessions(&user_id).await? {
            self.verify_session(&session_token).await?;
        }
        Ok(())
    }

    pub async fn refresh_session_token(&self, session_token: &SessionToken, user_id: &UserId) -> Result<()>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;

        redis_connection.expire(&format!("session_token:{}", session_token.to_string()), USER_SESSION_TIMEOUT_SECONDS).await?;

        let user_sessions_key = format!("user_sessions:{}", user_id.to_string());
        redis_connection.zadd(&user_sessions_key, session_token.to_string(), Utc::now().timestamp_millis()).await?;

        Ok(())
    }

    pub async fn get_user_from_session_token(&self, session_token: &SessionToken) -> Result<UserSession>{
        let mut redis_connection = self.application_redis.get_async_connection().await?;

        let session_json: String = redis_connection.get(&format!("session_token:{}", session_token.to_string())).await?;

        //println!("getting session_json: {}", session_json);

        let user_session: UserSession = serde_json::from_str(&session_json)?;

        // note: it may be needlessly expensive to do this every single time, presumably, users are doing this on the reg
        self.refresh_session_token(session_token, &user_session.user_id).await?;

        return Ok(user_session);
    }

}