use rocket::serde::uuid::Uuid;
use scylla::macros::FromRow;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct BasicThingCreate {
	#[validate(length(min = 4, max = 255))]
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
