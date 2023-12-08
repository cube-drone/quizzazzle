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
            "CREATE TABLE IF NOT EXISTS ks.user (id uuid PRIMARY KEY, name text, created_at timestamp)",
            &[],
        )
        .await?;

    let mut prepared_queries = HashMap::new();
    prepared_queries.insert(
        "create_basic",
        scylla_session
            .prepare("INSERT INTO ks.basic (id, name, created_at) VALUES (?, ?, ?)")
            .await?,
    );
    prepared_queries.insert(
        "get_basic",
        scylla_session
            .prepare("SELECT id, name, created_at FROM ks.basic WHERE id = ?")
            .await?,
    );

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
