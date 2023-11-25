use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use scylla::Session;
use scylla::macros::FromRow;
use scylla::prepared_statement::PreparedStatement;
//use scylla::transport::session::{IntoTypedRows};
use rocket::serde::uuid::Uuid;
use serde::{Serialize, Deserialize};

pub async fn _initialize(scylla_session: &Arc<Session>) -> Result<HashMap<&str, PreparedStatement>> {

	scylla_session.query("CREATE TABLE IF NOT EXISTS ks.basic (id uuid PRIMARY KEY, name text)", &[]).await?;

	let mut prepared_queries = HashMap::new();
	prepared_queries.insert("create_basic", scylla_session.prepare("INSERT INTO ks.basic (id, name) VALUES (?, ?)").await?);

	Ok(prepared_queries)
}

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct DatabaseBasicThing {
	pub id: Uuid,
	pub name: String,
}

pub async fn _create_basic(scylla_session: &Arc<Session>, uuid: &Uuid, text: &String) -> Result<()> {

	scylla_session.query("INSERT INTO ks.basic (id, name) VALUES (?, ?)", (uuid, text)).await?;

	Ok(())
}

pub async fn _get_basic(scylla_session: &Arc<Session>, uuid: &Uuid) -> Result<Option<DatabaseBasicThing>> {

	Ok(scylla_session.query("SELECT id, name FROM ks.basic WHERE id = ?", (uuid, )).await?.maybe_first_row_typed::<DatabaseBasicThing>()?)
}