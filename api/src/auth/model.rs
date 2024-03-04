use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use redis::AsyncCommands;

use anyhow::Result;
use anyhow::anyhow;

use rocket::serde::uuid::Uuid;
use scylla::prepared_statement::PreparedStatement;
//use scylla::frame::value::Timestamp;
use scylla::Session;
use chrono::Utc;


use crate::email::EmailAddress;
use crate::Services;
use crate::auth::hashes;

use crate::auth::tables::table_user;
use crate::auth::tables::table_user_email;
use crate::auth::tables::table_user_ip;
use crate::auth::tables::table_user_invite;

const ROOT_USER_ID: UserId = UserId(Uuid::from_u128(0));
const DEFAULT_THUMBNAIL_URL: &str = "/static/chismas.png";


pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut user_queries: HashMap<&'static str, PreparedStatement> = table_user::initialize(scylla_session).await?;
    let mut user_email_queries: HashMap<&'static str, PreparedStatement> = table_user_email::initialize(scylla_session).await?;
    let mut user_ip_queries: HashMap<&'static str, PreparedStatement> = table_user_ip::initialize(scylla_session).await?;
    let mut user_invite_queries: HashMap<&'static str, PreparedStatement> = table_user_invite::initialize(scylla_session).await?;

    let mut prepared_queries = HashMap::new();


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

    let queries_to_merge = vec![
        &mut user_queries,
        &mut user_email_queries,
        &mut user_ip_queries,
        &mut user_invite_queries,
    ];

    for query_map in queries_to_merge {
        prepared_queries.extend(query_map.drain());
    }

    Ok(prepared_queries)
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
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

/*
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Invite{
    pub invite_code: InviteCode,
    pub is_used: bool,
}
*/

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
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
    pub is_verified: bool,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub is_known_ip: bool,
    pub ip: IpAddr,
    pub tags: Vec<String>,
}

impl crate::services::auth_token_service::HasUserId for UserSession{
    fn user_id(&self) -> Uuid{
        self.user_id.to_uuid()
    }
}

impl UserSession {
    pub fn to_verified_user_session(&self) -> VerifiedUserSession {
        VerifiedUserSession {
            user_id: self.user_id,
            display_name: self.display_name.clone(),
            thumbnail_url: self.thumbnail_url.clone(),
            is_admin: self.is_admin,
            tags: self.tags.clone(),
        }
    }
    pub fn to_admin_user_session(&self) -> AdminUserSession {
        AdminUserSession {
            user_id: self.user_id,
            display_name: self.display_name.clone(),
            thumbnail_url: self.thumbnail_url.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifiedUserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
    pub is_admin: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminUserSession {
    pub user_id: UserId,
    pub display_name: String,
    pub thumbnail_url: String,
    pub tags: Vec<String>,
}


const INVITE_CODE_REGENERATION_TIME_MS: i64 = 86400 * 1000 * 4; // 4 days

impl table_user::UserDatabaseRaw {
    pub fn available_user_invites(&self) -> i32 {
        if self.is_admin {
            return 1000000;
        }
        if self.tags.contains(&"unlimited_invites".to_string()) {
            return 1000000;
        }
        let time_since_creation = Utc::now() - self.created_at;
        let time_in_ms = time_since_creation.num_milliseconds() as f64;
        let invite_codes = time_in_ms as f64 / INVITE_CODE_REGENERATION_TIME_MS as f64;
        let n_invite_codes: i32 = invite_codes.ceil() as i32;
        n_invite_codes
    }
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
        &self,
    ) -> Result<InviteCode> {
        // for testing, generate a new invite code from the root user
        Ok(InviteCode::new())
    }

    pub async fn create_invite_code(
        &self,
        user_id: &UserId,
    ) -> Result<()> {
        // first we need to check if the user has any available invites
        let user_maybe = self.table_user_get(&user_id).await?;
        let invite_count = self.table_user_invite_count(&user_id).await?;
        match user_maybe {
            None => {
                Err(anyhow!("User does not exist!"))
            },
            Some(user) => {
                let available_invites = user.available_user_invites();
                if available_invites < invite_count {
                    return Err(anyhow!("No available invites!"));
                }
                else{
                    self.table_user_invite_create(&user_id, &InviteCode::new()).await?;
                    Ok(())
                }
            }
        }
    }

    pub async fn delete_invite_code(
        &self,
        user_id: &UserId,
        invite_code: &InviteCode,
    ) -> Result<()> {
        let invite = self.table_user_invite_get(&invite_code).await?;

        match invite {
            None => {
                return Err(anyhow!("Invite code does not exist!"));
            },
            Some(invite) => {
                if invite.user_id != user_id.to_uuid() {
                    return Err(anyhow!("You can't delete that invite code! It's not yours!"));
                }
                self.table_user_invite_delete(&invite_code).await?;
                Ok(())
            }
        }
    }

    pub async fn get_my_invites(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<table_user_invite::UserInviteDatabaseRaw>> {
        // for testing, generate a new invite code from the root user
        self.table_user_invite_get_user(&user_id).await
    }

    pub async fn get_number_available_invites(
        &self,
        user_id: &UserId) -> Result<i32> {
        let user_maybe = self.table_user_get(user_id).await?;
        match user_maybe {
            None => {
                Err(anyhow!("User does not exist!"))
            },
            Some(user) => {
                Ok(user.available_user_invites())
            }
        }
    }

    pub async fn get_user_via_email(
        &self,
        email: &str,
    ) -> Result<Option<table_user::UserDatabaseRaw>> {

        let user_id_maybe = self.table_user_email_get_uuid(email).await?;

        if let Some(user_id) = user_id_maybe {
            return self.table_user_get(&user_id).await;
        }
        else{
            return Ok(None);
        }
    }

    pub async fn create_root_user(&self) -> Result<()>{
        // don't create a root user if one already exists
        if self.table_user_exists(&ROOT_USER_ID).await? {
            return Ok(());
        }

        let display_name = "root";
        let email = env::var("GROOVELET_ROOT_EMAIL").unwrap_or_else(|_| "root@gooble.email".to_string());
        let root_auth_password = env::var("GROOVELET_ROOT_AUTH_PASSWORD").unwrap_or_else(|_| "root".to_string());

        let hashed_password: String = hashes::password_hash_async(&root_auth_password).await?;

        self.table_user_create(
            &ROOT_USER_ID,
            display_name,
            None,
            &hashed_password,
            &email,
            true,
            true,
            DEFAULT_THUMBNAIL_URL,
        ).await?;

        self.table_user_email_create(
            &email,
            &ROOT_USER_ID,
        ).await?;

        Ok(())
    }

    pub async fn create_user(
        &self,
        user_create: UserCreate<'_>,
        ip: IpAddr,
    ) -> Result<SessionToken> {
        if self.table_user_exists(&user_create.user_id).await? {
            return Err(anyhow!("User somehow already exists! Wow, UUIDs are not as unique as I thought!"));
        }
        if !self.table_user_exists(&user_create.parent_id).await? {
            return Err(anyhow!("Parent user does not exist!"));
        }
        let existing_user_with_same_email = self.get_user_via_email(&user_create.email).await?;
        if let Some(existing_user_with_same_email) = existing_user_with_same_email {
            if existing_user_with_same_email.is_verified{
                return Err(anyhow!("Email already exists!"));
            }
            else{
                // TODO: delete the unverified user
                // and just create a new one, now
                // suck it, chump
                self.table_user_delete(&UserId::from_uuid(existing_user_with_same_email.id)).await?;
            }
        }

        let hashed_password: String = hashes::password_hash_async(&user_create.password).await?;

        // core user table!
        self.table_user_create(
            &user_create.user_id,
            user_create.display_name,
            Some(&user_create.parent_id),
            &hashed_password,
            user_create.email,
            user_create.is_verified,
            user_create.is_admin,
            DEFAULT_THUMBNAIL_URL).await?;

        self.table_user_email_create(
            user_create.email,
            &user_create.user_id
        ).await?;

        self.table_user_email_domain_create(
            user_create.email,
            &user_create.user_id
        ).await?;

        self.send_verification_email( &user_create.user_id, &user_create.email ).await?;

        let user_session = UserSession{
            user_id: user_create.user_id,
            display_name: user_create.display_name.to_string(),
            thumbnail_url: DEFAULT_THUMBNAIL_URL.to_string(),
            is_verified: user_create.is_verified,
            is_admin: user_create.is_admin,
            is_known_ip: true,
            ip: ip,
            tags: vec!["tag_default".to_string()],
        };

        let session_token = self.create_session_token(&user_session).await?;

        Ok(session_token)
    }

    pub async fn is_this_a_known_ip_for_this_user(
        &self,
        user_id: &UserId,
        ip: &IpAddr,
    ) -> Result<bool> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user_ip")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ip,),
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

    pub async fn login(&self, email: &str, password: &str, ip: IpAddr) -> Result<SessionToken> {
        let email_user = self.get_user_via_email(&email).await?;
        if let Some(email_user) = email_user {
            let password_success:bool = hashes::password_test_async(&password, &email_user.hashed_password).await?;

            let known_ip = self.is_this_a_known_ip_for_this_user(&UserId::from_uuid(email_user.id), &ip).await?;

            if !known_ip {
                self.send_ip_verification_email(&UserId::from_uuid(email_user.id), &email).await?;
            }

            if password_success {
                let user_id: UserId = UserId::from_uuid(email_user.id);
                let user_session: UserSession = UserSession{
                    user_id: user_id,
                    display_name: email_user.display_name,
                    thumbnail_url: email_user.thumbnail_url,
                    is_verified: email_user.is_verified,
                    is_admin: email_user.is_admin,
                    is_known_ip: known_ip,
                    ip: ip,
                    tags: email_user.tags,
                };

                let session_token = self.create_session_token(&user_session).await?;
                return Ok(session_token);
            }
        }
        Err(anyhow!("Invalid email or password!"))
    }

/*
          _______  _______ _________ _______ _________ _______  _______ __________________ _______  _
|\     /|(  ____ \(  ____ )\__   __/(  ____ \\__   __/(  ____ \(  ___  )\__   __/\__   __/(  ___  )( (    /|
| )   ( || (    \/| (    )|   ) (   | (    \/   ) (   | (    \/| (   ) |   ) (      ) (   | (   ) ||  \  ( |
| |   | || (__    | (____)|   | |   | (__       | |   | |      | (___) |   | |      | |   | |   | ||   \ | |
( (   ) )|  __)   |     __)   | |   |  __)      | |   | |      |  ___  |   | |      | |   | |   | || (\ \) |
 \ \_/ / | (      | (\ (      | |   | (         | |   | |      | (   ) |   | |      | |   | |   | || | \   |
  \   /  | (____/\| ) \ \_____) (___| )      ___) (___| (____/\| )   ( |   | |   ___) (___| (___) || )  \  |
   \_/   (_______/|/   \__/\_______/|/       \_______/(_______/|/     \|   )_(   \_______/(_______)|/    )_)

*/

    pub async fn test_get_last_email(&self, email_address: &str) -> Option<String> {
        let last_email_sent_key = format!("last_email_sent:${}", email_address);
        self.local_cache.get(&last_email_sent_key).await
    }

    pub async fn send_verification_email(
        &self,
        user_id: &UserId,
        email_address: &str,
    ) -> Result<()> {
        let email_verification_token = self.email_token_service.create_token(user_id.clone()).await?;

        let public_address = self.config_get_public_address();

        let email_verification_link = format!("{}/auth/verify_email?token={}", public_address, email_verification_token);

        self.email.send_verification_email(&EmailAddress::new(email_address.to_string())?, &email_verification_link).await?;

        // we keep track of the last email sent, so that we can test this functionality
        if ! self.is_production {
            let last_email_sent_key = format!("last_email_sent:${}", email_address);
            self.local_cache.insert(last_email_sent_key, email_verification_link).await;
        }

        Ok(())
    }

    pub async fn resend_verification_email(
        &self,
        user_id: &UserId,
    ) -> Result<()> {
        let user_maybe = self.table_user_get(user_id).await?;
        match user_maybe {
            None => {
                Err(anyhow!("User does not exist!"))
            },
            Some(user) => {
                if user.is_verified {
                    Err(anyhow!("User is already verified!"))
                }
                else{
                    self.send_verification_email(&user_id, &user.email).await?;
                    Ok(())
                }
            }
        }
    }

    pub async fn send_ip_verification_email(
        &self,
        user_id: &UserId,
        email_address: &str,
    ) -> Result<()> {
        let email_verification_token = self.ip_token_service.create_token(user_id.clone()).await?;

        let public_address = self.config_get_public_address();

        let ip_verification_link = format!("{}/auth/verify_ip?token={}", public_address, email_verification_token);

        self.email.send_ip_verification_email(&EmailAddress::new(email_address.to_string())?, &ip_verification_link).await?;

        if ! self.is_production {
            let last_email_sent_key = format!("last_email_sent:${}", email_address);
            self.local_cache.insert(last_email_sent_key, ip_verification_link).await;
        }

        Ok(())
    }

    pub async fn resend_ip_verification_email(
        &self,
        user_id: &UserId,
    ) -> Result<()> {
        let user_maybe = self.table_user_get(user_id).await?;
        match user_maybe {
            None => {
                Err(anyhow!("User does not exist!"))
            },
            Some(user) => {
                self.send_ip_verification_email(&user_id, &user.email).await?;
                Ok(())
            }
        }
    }

    pub async fn verify_email(
        &self,
        email_verification_token: &Uuid,
    ) -> Result<UserId> {
        let user_id = self.email_token_service.get_token(&email_verification_token).await?;
        match user_id {
            None => {
                Err(anyhow!("Invalid token!"))
            },
            Some(user_id) => {
                if ! self.table_user_exists(&user_id).await? {
                    return Err(anyhow!("User does not exist!"));
                }

                self.table_user_verify(&user_id).await?;

                self.verify_all_sessions(&user_id).await?;

                self.email_token_service.delete_token(&email_verification_token).await?;

                Ok(user_id)
            }
        }
    }

    pub async fn remember_ip(
        &self,
        user_id: &UserId,
        ip: &IpAddr,
    ) -> Result<()> {
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_user_ip")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ip, ),
            )
            .await?;

        Ok(())
    }

    pub async fn verify_ip(
        &self,
        email_verification_token: &Uuid,
        ip: &IpAddr,
    ) -> Result<()> {
        let user_id = self.ip_token_service.get_token(&email_verification_token).await?;
        match user_id {
            None => {
                Err(anyhow!("Invalid token!"))
            },
            Some(user_id) => {
                if ! self.table_user_exists(&user_id).await? {
                    return Err(anyhow!("User does not exist!"));
                }

                self.table_user_ip_create(&user_id, &ip).await?;

                self.verify_ip_all_sessions(&user_id, &ip).await?;

                self.ip_token_service.delete_token(&email_verification_token).await?;

                Ok(())
            }
        }
    }

    pub async fn forget_ip(
        &self,
        user_id: &UserId,
        ip: &IpAddr,
    ) -> Result<()> {
        self.table_user_ip_delete(&user_id, &ip).await?;

        Ok(())
    }

/*
                                 _                    _
 ___ ___ ___ ___ _ _ _ ___ ___ _| |   ___ ___ ___ ___| |_
| . | .'|_ -|_ -| | | | . |  _| . |  |  _| -_|_ -| -_|  _|
|  _|__,|___|___|_____|___|_| |___|  |_| |___|___|___|_|
|_|
*/

    pub async fn send_password_reset_email(
        &self,
        email_address: &str,
    ) -> Result<()> {

        let user_maybe = self.get_user_via_email(&email_address).await?;
        match user_maybe {
            None => {
                Err(anyhow!("User does not exist!"))
            },
            Some(user) => {
                let user_id = user.id;

                let password_reset_token = self.password_token_service.create_token(UserId::from_uuid(user_id)).await?;

                let public_address = self.config_get_public_address();

                let password_reset_link = format!("{}/auth/password_reset/stage_2?token={}", public_address, password_reset_token);

                self.email.send_password_reset_email(&EmailAddress::new(email_address.to_string())?, &password_reset_link).await?;

                if ! self.is_production {
                    let last_email_sent_key = format!("last_email_sent:${}", email_address);
                    self.local_cache.insert(last_email_sent_key, password_reset_link).await;
                }

                Ok(())
            }
        }
    }

    pub async fn password_reset(&self, password_token: &Uuid, password: &str, ip: &IpAddr) -> Result<SessionToken> {
        // 1. verify the token and find the associated user id
        let user_id_maybe = self.password_token_service.get_token(&password_token).await?;

        match user_id_maybe{
            None => {
                Err(anyhow!("Invalid token!"))
            },
            Some(user_id) => {
                if ! self.table_user_exists(&user_id).await? {
                    return Err(anyhow!("User does not exist!"));
                }

                // 2. hash the password and save it against the associated user id
                let hashed_password: String = hashes::password_hash_async(&password).await?;

                self.table_user_password_change(&user_id, &hashed_password).await?;

                // 3. while we're here, save that IP as a known IP for this user
                self.remember_ip(&user_id, &ip).await?;

                // 4. get that user, create a session token, and return it
                let user = self.table_user_get(&user_id).await?.unwrap();
                let user_session = UserSession{
                    user_id: UserId::from_uuid(user.id),
                    display_name: user.display_name,
                    thumbnail_url: user.thumbnail_url,
                    is_verified: user.is_verified,
                    is_admin: user.is_admin,
                    is_known_ip: true,
                    ip: *ip,
                    tags: user.tags,
                };

                let session_token = self.create_session_token(&user_session).await?;

                Ok(session_token)
            }
        }
    }


/*
 ______     ______     ______   ______        __         __     __    __     __     ______   ______
/\  == \   /\  __ \   /\__  _\ /\  ___\      /\ \       /\ \   /\ "-./  \   /\ \   /\__  _\ /\  ___\
\ \  __<   \ \  __ \  \/_/\ \/ \ \  __\      \ \ \____  \ \ \  \ \ \-./\ \  \ \ \  \/_/\ \/ \ \___  \
 \ \_\ \_\  \ \_\ \_\    \ \_\  \ \_____\     \ \_____\  \ \_\  \ \_\ \ \_\  \ \_\    \ \_\  \/\_____\
  \/_/ /_/   \/_/\/_/     \/_/   \/_____/      \/_____/   \/_/   \/_/  \/_/   \/_/     \/_/   \/_____/

*/


    pub async fn rate_limit(&self, key: &String, requests_per_hour: usize) -> Result<()> {
        /*
            Whatever the key is, it's not allowed to call this function more than requests_per_hour times per hour,
            if it does, it'll throw a rate limit error.
            It also can't call this function more than once every 5 seconds.
        */
        let mut redis_connection = self.application_redis.get_async_connection().await?;

        // everything has a 5-second rate limit by default
        let rate_limit_key = format!("rate_limit:${}", key);
        let rate_limit_exists: bool = redis_connection.exists(&rate_limit_key).await?;
        if rate_limit_exists {
            return Err(anyhow!("Rate limit exceeded!"));
        }
        redis_connection.set_ex(&rate_limit_key, "NO", 5).await?;

        // everything also gets no more than requests_per_hour requests per hour
        let rate_limit_key = format!("rate_limit_hour:${}", key);
        let rate_limit_exists: bool = redis_connection.exists(&rate_limit_key).await?;
        if !rate_limit_exists {
            redis_connection.set_ex(&rate_limit_key, 0, 3600).await?;
        }
        else{
            let rate_limit_count: usize = redis_connection.incr(&rate_limit_key, 1).await?;
            if rate_limit_count > requests_per_hour {
                return Err(anyhow!("Rate limit exceeded!"));
            }
        }

        Ok(())
    }

    pub async fn rate_limits(&self, keys: &Vec<String>, requests_per_hour: usize) -> Result<()> {
        /*
            Apply multiple rate limits at once.
        */
        for key in keys {
            self.rate_limit(key, requests_per_hour).await?;
        }
        Ok(())
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

        let session_token = self.auth_token_service.create_token(&user_session.user_id.to_uuid(), user_session).await?;

        Ok(SessionToken(session_token))
    }

    /*
    pub async fn logout(&self, session_token: &SessionToken) -> Result<()>{
        let user_session = self.get_user_from_session_token(&session_token).await?;
        let user_id = user_session.user_id;
        self.delete_session(&session_token, &user_id).await?;
        Ok(())
    }
    */

    pub async fn delete_session(&self, session_token: &SessionToken, _user_id: &UserId) -> Result<()>{
        self.auth_token_service.delete_token(&session_token.to_uuid()).await?;

        Ok(())
    }

    pub async fn delete_all_sessions(&self, user_id: &UserId) -> Result<()>{
        self.auth_token_service.clear_tokens(&user_id.to_uuid()).await?;

        Ok(())
    }

    pub async fn verify_all_sessions(&self, user_id: &UserId) -> Result<()>{
        /*
            after the user has verified that their email address is valid, we should verify all of their sessions
         */

        let sessions = self.auth_token_service.get_tokens(&user_id.to_uuid()).await?;

        for(session_token, maybe_session) in sessions{
            match maybe_session{
                None => {},
                Some(mut user_session) => {
                    user_session.is_verified = true;
                    self.auth_token_service.update_token(&session_token, user_session).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn verify_ip_all_sessions(&self, user_id: &UserId, ip: &IpAddr) -> Result<()>{
        /*
            after the user has verified that their email address is valid, we should verify all of their sessions
         */
        let sessions = self.auth_token_service.get_tokens(&user_id.to_uuid()).await?;

        for(session_token, maybe_session) in sessions{
            match maybe_session{
                None => {},
                Some(mut user_session) => {
                    if user_session.ip.to_string() == ip.to_string() {
                        user_session.is_known_ip = true;
                        self.auth_token_service.update_token(&session_token, user_session).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn get_user_from_session_token(&self, session_token: &SessionToken) -> Result<Option<UserSession>>{
        let user_session = self.auth_token_service.get_token(&session_token.to_uuid()).await?;

        Ok(user_session)
    }

}