use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;

use anyhow::Result;
use futures::join;
use rocket::serde::uuid::Uuid;
use rocket::tokio;

use moka::future::Cache;
use rusqlite::{Connection as SqlConnection, DatabaseName, params};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use crate::services::background_tick::RequiresBackgroundTick;

pub struct UserDatabaseRaw {
    pub id: Uuid,
    pub display_name: String,
    pub parent_id: Option<Uuid>,
    pub hashed_password: String,
    pub email: String,
    pub thumbnail_url: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub tags: Vec<String>,
    pub opcount: i32,
    pub logincount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}