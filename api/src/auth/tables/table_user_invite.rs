use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use scylla::Session;
use scylla::macros::FromRow;
use scylla::prepared_statement::PreparedStatement;
use chrono::{Utc, Duration};

use rocket::serde::uuid::Uuid;

use crate::Services;
use crate::auth::model::{UserId, InviteCode, Invite};

#[derive(FromRow)]
pub struct UserInviteDatabaseRaw {
    pub user_id: Uuid,
    pub invite_key: Uuid,
    pub created_at: Duration,
    pub is_used: bool,
    pub used_at: Option<Duration>,
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
                created_at timestamp,
                used_at timestamp
            );
            "#, &[], ).await?;

    println!("set_user_invite");
    prepared_queries.insert(
        "set_user_invite",
        scylla_session
            .prepare("INSERT INTO ks.user_invite (user_id, invite_code, created_at, is_used) VALUES (?, ?, ?, false);")
            .await?,
    );
    /*
        println!("does_invite_exist");
        prepared_queries.insert(
            "does_invite_exist",
            scylla_session
                .prepare("SELECT invite_code FROM ks.user_invite WHERE user_id = ? AND invite_code = ?;")
                .await?,
        );
     */

    println!("use_invite");
    prepared_queries.insert(
        "use_invite",
        scylla_session
            .prepare("UPDATE ks.user_invite SET is_used = true, used_at = ? WHERE invite_code = ?;")
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

        // user -> ip
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

        Ok(())
    }

    pub async fn table_user_invite_use(
        &self,
        user_id: &UserId,
        invite_code: &InviteCode,
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
                (invite_code.to_uuid(),),
            ).await?;

        Ok(())
    }

    pub async fn table_user_invite_exists(
        &self,
        user_id: &UserId,
        invite_code: &InviteCode,
    ) -> Result<bool> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("does_invite_exist")
                    .expect("Query missing!"),
                (user_id.to_uuid(), invite_code.to_uuid(),),
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

    pub async fn table_user_invite_get(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Invite>> {
        let rows = self
            .scylla
            .session
            .query(
                "SELECT invite_key, is_used FROM ks.user_invite WHERE user_id = ?;",
                (user_id.to_uuid(),),
            )
            .await?;

        let mut invite_codes = Vec::new();
        for row in rows.rows_typed::<UserInviteDatabaseRaw>()? {
            let row_data = row?;
            let invite_code = InviteCode::from_uuid(row_data.invite_key);
            let is_used = row_data.is_used;
            invite_codes.push(Invite{invite_code, is_used});
        }

        Ok(invite_codes)
    }


}