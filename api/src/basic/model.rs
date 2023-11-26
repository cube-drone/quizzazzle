use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use scylla::Session;
use scylla::prepared_statement::PreparedStatement;
use rocket::serde::uuid::Uuid;

use crate::basic::types::BasicThingDatabase;

use crate::ScyllaService;

pub async fn initialize(scylla_session: &Arc<Session>) -> Result<HashMap<&'static str, PreparedStatement>> {

	scylla_session.query("CREATE TABLE IF NOT EXISTS ks.basic (id uuid PRIMARY KEY, name text)", &[]).await?;

	let mut prepared_queries = HashMap::new();
	prepared_queries.insert("create_basic", scylla_session.prepare("INSERT INTO ks.basic (id, name) VALUES (?, ?)").await?);

	Ok(prepared_queries)
}

pub async fn create_basic_thing(scylla: &ScyllaService, basic_thing: &BasicThingDatabase) -> Result<()> {
	scylla.session.query("INSERT INTO ks.basic (id, name) VALUES (?, ?)", (basic_thing.id, basic_thing.name.clone())).await?;

	Ok(())
}

pub async fn get_basic(scylla: &ScyllaService, uuid: &Uuid) -> Result<Option<BasicThingDatabase>> {
	Ok(scylla.session.query("SELECT id, name FROM ks.basic WHERE id = ?", (uuid, )).await?.maybe_first_row_typed::<BasicThingDatabase>()?)
}