use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use rocket::serde::uuid::Uuid;
use scylla::prepared_statement::PreparedStatement;
use scylla::frame::value::Timestamp;
use scylla::Session;

use crate::basic::types::BasicThingDatabase;

use crate::ScyllaService;

pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {
    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.user (" +
                "id uuid PRIMARY KEY, " +
                "parent_id uuid, " +
                "has_password boolean, " +
                "hashed_password text, " +
                "thumbnail_url text, " +
                "created_at timestamp, " +
                "updated_at timestamp)",
            &[],
        )
        .await?;

    let mut prepared_queries = HashMap::new();
    prepared_queries.insert(
        "create_blank_user",
        scylla_session
            .prepare("INSERT INTO ks.user (id, parent_id, has_password, created_at, updated_at) VALUES (?, ?, false, ?, ?);")
            .await?,
    );

    prepared_queries.insert(
        "update_user_password",
        scylla_session
            .prepare("UPDATE ks.user USING TTL 0 SET hashed_password = ? WHERE id = ?;")
            .await?,
    );

    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.user_parents (" +
                "user_id uuid PRIMARY KEY, " +
                "parents list<uuid>)",
            &[],
        )
        .await?;

    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.user_ips (" +
                "user_id uuid PRIMARY KEY, " +
                "ips list<uuid>)",
            &[],
        )

    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.user_email (" +
                "user_id uuid, " +
                "email text, " +
                "email_domain text, " +
                "primary_email boolean, " +
                "verified boolean, " +
                "verification_token text, " +
                "created_at timestamp, " +
                "updated_at timestamp, " +
                "PRIMARY KEY (user_id, email))"
            &[],
        )
        .await?;

    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.email_user (" +
                "email text PRIMARY KEY, " +
                "user_id uuid)"
            &[],
        )
        .await?;

    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.email_domain (" +
                "email_domain text" +
                "user_id uuid, " +
                "PRIMARY KEY (email_domain, user_id))"
            &[],
        )
        .await?;

    Ok(prepared_queries)
}

pub async fn create_anon_session(redis: &ClusterClient) -> Result<String> {
    let mut redis_connection = redis.get_async_connection().await?;
    let token: String = nanoid!(32);
    let _: () = redis_connection
        .set_ex(token.clone(), "anon", 60 * 60 * 24 * 7)
        .await?;

    Ok(token)
}
