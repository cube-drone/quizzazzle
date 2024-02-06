use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use rocket::serde::uuid::Uuid;
use scylla::Session;
use scylla::prepared_statement::PreparedStatement;
use scylla::macros::FromRow;
use chrono::{Utc, Duration};

use crate::{email, Services};
use crate::auth::model::UserId;


pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {

    let mut prepared_queries = HashMap::new();

    // email --> user
    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_user (
                email text PRIMARY KEY,
                user_id uuid)
            "#, &[], ).await?;

        prepared_queries.insert(
            "get_email_user",
            scylla_session
                .prepare("SELECT user_id FROM ks.email_user WHERE email = ?;")
                .await?,
        );

        prepared_queries.insert(
            "set_email_user",
            scylla_session
                .prepare("INSERT INTO ks.email_user (email, user_id) VALUES (?, ?);")
                .await?,
        );

    // email_domain --> user
    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.email_domain (
                email_domain text,
                user_id uuid,
                PRIMARY KEY (email_domain, user_id))
            "#, &[], ).await?;

        // TODO: limit and paginate this query
        /*
        prepared_queries.insert(
            "get_email_domain_users",
            scylla_session
                .prepare("SELECT user_id FROM ks.email_domain WHERE email_domain = ?;")
                .await?,
        );
         */

        prepared_queries.insert(
            "set_email_domain_user",
            scylla_session
                .prepare("INSERT INTO ks.email_domain (email_domain, user_id) VALUES (?, ?);")
                .await?,
        );


    Ok(prepared_queries)
}

impl Services {

    pub async fn table_user_email_create(
        &self,
        email: &str,
        user_id: &UserId,
    ) -> Result<()> {
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_email_user")
                    .expect("Query missing! (set_email_user)"),
                (email, user_id.to_uuid()),
            )
            .await?;
        Ok(())
    }

    pub async fn table_user_email_get_uuid(
        &self,
        email: &str,
    ) -> Result<Option<UserId>> {
        let result = self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("get_email_user")
                    .expect("Query missing! (get_email_user)"),
                (email,),
            )
            .await?;

        if let Some(rows) = result.rows {
            if rows.len() > 0 {
                let row = rows.get(0).unwrap();
                let user_id: Uuid = row.columns[0].as_ref().unwrap().as_uuid().unwrap();
                let user_id = UserId::from_uuid(user_id);
                return Ok(Some(user_id));
            }
            else{
                return Ok(None);
            }
        }
        else{
            return Ok(None);
        }
    }

    pub async fn table_user_email_domain_create(
        &self,
        email: &str,
        user_id: &UserId,
    ) -> Result<()> {
        let email_domain = email.split('@').last().unwrap();

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_email_domain_user")
                    .expect("Query missing! (set_email_domain_user)"),
                (email_domain, user_id.to_uuid()),
            )
            .await?;
        Ok(())
    }

}