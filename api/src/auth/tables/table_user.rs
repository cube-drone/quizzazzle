use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use rocket::serde::uuid::Uuid;
use scylla::Session;
use scylla::prepared_statement::PreparedStatement;
use scylla::macros::FromRow;
use chrono::{Utc, Duration};

use crate::Services;
use crate::auth::model::UserId;


pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut prepared_queries = HashMap::new();
    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user (
                id uuid PRIMARY KEY,
                display_name text,
                parent_id uuid,
                hashed_password text,
                thumbnail_url text,
                email text,
                is_verified boolean,
                is_admin boolean,
                tags set<text>,
                opcount int,
                logincount int,
                invitecount int,
                created_at timestamp,
                updated_at timestamp);
        "#, &[], ).await?;

        prepared_queries.insert(
            "create_user",
            scylla_session
                .prepare(r#"INSERT INTO ks.user (
                    id,
                    display_name,
                    parent_id,
                    hashed_password,
                    email,
                    thumbnail_url,
                    is_verified,
                    is_admin,
                    tags,
                    opcount,
                    logincount,
                    invitecount,
                    created_at,
                    updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, {'tag_default'}, 0, 0, 0, ?, ?);"#
                )
                .await?,
        );

        prepared_queries.insert(
            "get_user_exists",
            scylla_session
                .prepare("SELECT id FROM ks.user WHERE id = ?;")
                .await?,
        );

        prepared_queries.insert(
            "get_user",
            scylla_session
                .prepare("SELECT id, display_name, parent_id, hashed_password, email, thumbnail_url, is_verified, is_admin, tags, opcount, logincount, invitecount, created_at, updated_at FROM ks.user WHERE id = ?;")
                .await?,
        );

        prepared_queries.insert(
            "verify_user_email",
            scylla_session
                .prepare("UPDATE ks.user SET is_verified = true WHERE id = ?;")
                .await?,
        );

        prepared_queries.insert(
            "change_user_password",
            scylla_session
                .prepare("UPDATE ks.user SET hashed_password = ? WHERE id = ?;")
                .await?,
        );

        prepared_queries.insert(
            "delete_user",
            scylla_session
                .prepare("DELETE FROM ks.user WHERE id = ?;")
                .await?,
        );

    Ok(prepared_queries)
}

#[derive(FromRow)]
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
    pub invitecount: i32,
    pub opcount: i32,
    pub logincount: i32,
    pub created_at: Duration,
    pub updated_at: Duration,
}

impl Services {

    pub async fn table_user_exists(
        &self,
        user_id: &UserId,
    ) -> Result<bool> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
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

    pub async fn table_user_create(
        &self,
        user_id: &UserId,
        display_name: &str,
        parent_id: Option<&UserId>,
        hashed_password: &str,
        email: &str,
        is_verified: bool,
        is_admin: bool,
        thumbnail_url: &str,
    ) -> Result<()> {

        match parent_id {
            Some(pid) => {
                self.scylla
                    .session
                    .execute(
                        &self
                            .scylla
                            .prepared_queries
                            .get("create_user")
                            .expect("Query missing!"),
                        (user_id.to_uuid(), display_name, pid.to_uuid(), hashed_password, email, thumbnail_url, is_verified, is_admin, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
                    )
                    .await?;
            },
            None => {
                self.scylla
                    .session
                    .execute(
                        &self
                            .scylla
                            .prepared_queries
                            .get("create_user")
                            .expect("Query missing!"),
                        (user_id.to_uuid(), display_name, "", hashed_password, email, thumbnail_url, is_verified, is_admin, Utc::now().timestamp_millis(), Utc::now().timestamp_millis()),
                    )
                    .await?;
            },
        }

        Ok(())
    }

    pub async fn table_user_get(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserDatabaseRaw>> {
        Ok(self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
            )
            .await?
            .maybe_first_row_typed::<UserDatabaseRaw>()?)
    }

    pub async fn table_user_verify(
        &self,
        user_id: &UserId,
    ) -> Result<()> {
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("verify_user_email")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
            )
            .await?;
        Ok(())
    }

    pub async fn table_user_password_change(
        &self,
        user_id: &UserId,
        hashed_password: &str,
    ) -> Result<()> {
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("change_user_password")
                    .expect("Query missing!"),
                (hashed_password, user_id.to_uuid(),),
            ).await?;
        Ok(())
    }

    pub async fn table_user_delete(
        &self,
        user_id: &UserId,
    ) -> Result<()> {
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("delete_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(),),
            )
            .await?;

        Ok(())
    }

}