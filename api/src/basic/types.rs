use rocket::serde::uuid::Uuid;
use scylla::macros::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicThingCreate {
	pub name: String,
}

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct BasicThingDatabase {
	pub id: Uuid,
	pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicThingPublic {
	pub id: Uuid,
	pub name: String,
}
