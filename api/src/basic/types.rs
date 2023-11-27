use rocket::serde::uuid::Uuid;
use scylla::macros::FromRow;
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::Duration;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct BasicThingCreate {
    #[validate(length(min = 4, max = 255))]
    pub name: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct BasicThingDatabase {
    pub id: Uuid,
    pub name: String,
    pub created_at: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicThingPublic {
    pub id: Uuid,
    pub name: String,
    pub created_at: String, // we always want to share timestamps as ISO-8601 strings
}
