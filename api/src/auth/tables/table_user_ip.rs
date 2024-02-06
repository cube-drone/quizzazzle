
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

    // user --> ip
    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.user_ips (
                user_id uuid,
                ip inet,
                PRIMARY KEY(user_id, ip));
            "#, &[], ).await?;

    scylla_session
        .query(r#"
            CREATE TABLE IF NOT EXISTS ks.ip_users (
                ip inet,
                user_id uuid,
                PRIMARY KEY(ip, user_id));
            "#, &[], ).await?;

        /* we don't have a plan for this one yet: get all of the ips for a given user */
        /*
            prepared_queries.insert(
                "get_user_ips",
                scylla_session
                    .prepare("SELECT ip FROM ks.user_ips WHERE user_id = ?;")
                    .await?,
            );
         */

        // register an ip against a user
        // this lasts forever: if you've _ever_ logged in from an IP, it's good forever
        prepared_queries.insert(
            "set_user_ip",
            scylla_session
                .prepare("INSERT INTO ks.user_ips (user_id, ip) VALUES (?, ?);")
                .await?,
        );
        prepared_queries.insert(
            "set_ip_user",
            scylla_session
                .prepare("INSERT INTO ks.ip_users (ip, user_id) VALUES (?, ?);")
                .await?,
        );

        prepared_queries.insert(
            "delete_user_ip",
            scylla_session
                .prepare("DELETE FROM ks.user_ips WHERE user_id = ? AND ip = ?;")
                .await?,
        );

        prepared_queries.insert(
            "delete_ip_user",
            scylla_session
                .prepare("DELETE FROM ks.ip_users WHERE user_id = ? AND ip = ?;")
                .await?,
        );

        // this one's mostly here to test whether or not any given ip is "known" to us
        // if not, we need to send a verification email
        prepared_queries.insert(
            "get_user_ip",
            scylla_session
                .prepare("SELECT ip FROM ks.user_ips WHERE user_id = ? AND ip = ?;")
                .await?,
        );



    Ok(prepared_queries)
}

impl Services {

    pub async fn table_user_ip_create(
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
                    .get("set_user_ip")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ip, ),
            )
            .await?;
        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("set_ip_user")
                    .expect("Query missing!"),
                (ip, user_id.to_uuid(), ),
            )
            .await?;

        Ok(())
    }

    pub async fn table_user_ip_delete(
        &self,
        user_id: &UserId,
        ip: &IpAddr
    ) -> Result<()> {

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("delete_user_ip")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ip,),
            ).await?;

        self.scylla
            .session
            .execute(
                &self
                    .scylla
                    .prepared_queries
                    .get("delete_ip_user")
                    .expect("Query missing!"),
                (user_id.to_uuid(), ip,),
            ).await?;

        Ok(())
    }


}