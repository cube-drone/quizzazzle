use std::collections::HashMap;
use std::sync::Arc;
use std::net::IpAddr;
use anyhow::Result;
use scylla::Session;
use scylla::prepared_statement::PreparedStatement;

use crate::Services;
use crate::auth::model::UserId;


pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut prepared_queries = HashMap::new();

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_invite (
                user_id uuid PRIMARY KEY,
                invite_key uuid,
                created_at timestamp,
                is_used boolean,
                used_at timestamp
            );
            "#, &[], ).await?;

    prepared_queries.insert(
        "set_user_invite",
        scylla_session
            .prepare("INSERT INTO ks.user_invite (user_id, invite_key, created_at, is_used) VALUES (?, ?, ?, false);")
            .await?,
    );

    prepared_queries.insert(
        "use_invite",
        scylla_session
            .prepare("UPDATE ks.user_invite SET is_verified = true, used_at = ? WHERE user_id = ? AND invite_key = ?;")
            .await?,
    );

    Ok(prepared_queries)
}

impl Services {

    pub async fn table_user_invite_create(
        &self,
        user_id: &UserId,
        ip: &IpAddr

    ) -> Result<()> {

        // user -> ip
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_user_invite")
                    .expect("Query missing!"),
                (user_id.to_uuid(), invite_key, created_at, is_used),
            )
            .await?;

        Ok(())
    }

    pub async fn table_user_invite_use(
        &self,
        user_id: &UserId,
        invite_code: &InviteCode,
    ) -> Result<()> {
        let invite_key = invite_code.to_uuid();

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("use_user_invite")
                    .expect("Query missing!"),
                (used_at, user_id.to_uuid(), invite_key,),
            ).await?;

        Ok(())
    }


}