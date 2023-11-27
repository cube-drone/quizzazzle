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
            "CREATE TABLE IF NOT EXISTS ks.basic (id uuid PRIMARY KEY, name text, created_at timestamp)",
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

pub async fn create_basic_thing(
    scylla: &ScyllaService,
    basic_thing: &BasicThingDatabase,
) -> Result<()> {
    scylla
        .session
        .execute(
            &scylla
                .prepared_queries
                .get("create_basic")
                .expect("Query missing!"),
            (basic_thing.id, basic_thing.name.clone(), Timestamp::from(basic_thing.created_at)),
        )
        .await?;

    Ok(())
}

pub async fn get_basic(scylla: &ScyllaService, uuid: &Uuid) -> Result<Option<BasicThingDatabase>> {
    Ok(scylla
        .session
        .execute(
            &scylla
                .prepared_queries
                .get("get_basic")
                .expect("Query missing!"),
            (uuid,),
        )
        .await?
        .maybe_first_row_typed::<BasicThingDatabase>()?)
}
