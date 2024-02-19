use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use scylla::Session;
use scylla::macros::FromRow;
use scylla::prepared_statement::PreparedStatement;
use chrono::{Utc, DateTime};

use rocket::serde::uuid::Uuid;
use serde::Serialize;

use crate::Services;
use crate::auth::model::{UserId, InviteCode};

#[derive(FromRow, Serialize)]
pub struct UserInviteDatabaseRaw {
    pub invite_code: Uuid,
    pub user_id: Uuid,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut prepared_queries = HashMap::new();

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_invite (
                invite_code uuid PRIMARY KEY,
                user_id uuid,
                is_used boolean,
                used_by uuid,
                created_at timestamp,
                used_at timestamp
            );
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_invite_by_user (
                user_id uuid,
                invite_code uuid,
                PRIMARY KEY (user_id, invite_code)
            );
            "#, &[], ).await?;

    prepared_queries.insert(
        "set_user_invite",
        scylla_session
            .prepare("INSERT INTO ks.user_invite (user_id, invite_code, created_at, is_used) VALUES (?, ?, ?, false);")
            .await?,
    );

    prepared_queries.insert(
        "set_user_invite_by_user",
        scylla_session
            .prepare("INSERT INTO ks.user_invite_by_user (user_id, invite_code) VALUES (?, ?);")
            .await?,
    );

    prepared_queries.insert(
        "use_invite",
        scylla_session
            .prepare("UPDATE ks.user_invite SET is_used = true, used_at = ?, used_by = ? WHERE invite_code = ?;")
            .await?,
    );

    prepared_queries.insert(
        "get_user_invite_codes",
        scylla_session
            .prepare("SELECT invite_code FROM ks.user_invite_by_user WHERE user_id = ?;")
            .await?,
    );

    prepared_queries.insert(
        "get_user_invite_count",
        scylla_session
            .prepare("SELECT COUNT(*) FROM ks.user_invite_by_user WHERE user_id = ?;")
            .await?,
    );

    prepared_queries.insert(
        "get_user_invite_code",
        scylla_session
            .prepare("SELECT invite_code, user_id, is_used, created_at, used_at FROM ks.user_invite WHERE invite_code = ?;")
            .await?,
    );

    Ok(prepared_queries)
}

impl Services {

    pub async fn table_user_invite_create(
        &self,
        user_id: &UserId,
        invite_code: &InviteCode
    ) -> Result<()> {
        let created_at = Utc::now().timestamp_millis();

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_user_invite")
                    .expect("Query missing!"),
                (user_id.to_uuid(), invite_code.to_uuid(), created_at),
            )
            .await?;

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_user_invite_by_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(), invite_code.to_uuid()),
            )
            .await?;

        Ok(())
    }

    pub async fn table_user_invite_use(
        &self,
        invite_code: &InviteCode,
        used_by: &UserId,
    ) -> Result<()> {
        let used_at = Utc::now().timestamp_millis();

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("use_user_invite")
                    .expect("Query missing!"),
                (used_at, used_by.to_uuid(), invite_code.to_uuid(),),
            ).await?;

        Ok(())
    }

    pub async fn table_user_invite_count(
        &self,
        user_id: &UserId,
    ) -> Result<i32> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user_invite_count")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ),
            ).await?;

        Ok(result.rows.unwrap().len() as i32)
    }

    pub async fn table_user_invite_get(
        &self,
        invite_code: &InviteCode,
    ) -> Result<Option<UserInviteDatabaseRaw>> {
        Ok(self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user_invite_code")
                    .expect("Query missing!"),
                (invite_code.to_uuid(), ),
            ).await?
            .maybe_first_row_typed::<UserInviteDatabaseRaw>()?)
    }

    pub async fn table_user_invite_exists(
        &self,
        invite_code: &InviteCode,
    ) -> Result<bool> {
        Ok(self.table_user_invite_get(invite_code).await?.is_some())
    }

    pub async fn table_user_invite_get_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<UserInviteDatabaseRaw>> {
        let mut invites: Vec<UserInviteDatabaseRaw> = Vec::new();

        if let Some(rows) = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_user_invite_codes")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ),
            ).await?.rows {
                for row in rows {
                    let (invite_code,): (Uuid,) = row.into_typed::<(Uuid,)>()?;
                    let invite = self.table_user_invite_get(&InviteCode::from_uuid(invite_code)).await?;
                    if let Some(invite) = invite {
                        invites.push(invite);
                    }
                }
            }

        Ok(invites)
    }
}