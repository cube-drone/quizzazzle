use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;

use crate::Services;
use crate::ConfigService;
use crate::ScyllaService;

pub async fn initialize(
    scylla_session: &Arc<Session>,
) -> Result<HashMap<&'static str, PreparedStatement>> {
    scylla_session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.config (org text PRIMARY KEY, key text, value text)",
            &[],
        )
        .await?;

    let mut prepared_queries = HashMap::new();
    prepared_queries.insert(
        "create_config",
        scylla_session
            .prepare("INSERT INTO ks.config (org, key, value) VALUES ('default', ?, ?)")
            .await?,
    );
    prepared_queries.insert(
        "get_all_config",
        scylla_session
            .prepare("SELECT key, value FROM ks.config WHERE org = 'default'")
            .await?,
    );

    Ok(prepared_queries)
}

pub async fn _create_config(
    scylla: &ScyllaService,
    key: &String,
    value: &String,
) -> Result<()> {
    scylla
        .session
        .execute(
            &scylla
                .prepared_queries
                .get("create_config")
                .expect("Query missing!"),
            (key.clone(), value.clone()),
        )
        .await?;

    Ok(())
}

pub async fn update_config(services: & Services) -> Result<()> {

    let mut private_config: HashMap<String, String> = HashMap::new();
    let mut public_config: HashMap<String, String> = HashMap::new();

    private_config.insert("private_key".to_string(), "private_value".to_string());
    public_config.insert("public_key".to_string(), "public_value".to_string());

    // update the private and public config from scylla

    // Update the config (it has to be locked because it is shared between threads)
    services.config.write().unwrap().private_config = private_config;
    services.config.write().unwrap().public_config = public_config;

    /*
    Ok(scylla
        .session
        .execute(
            &scylla
                .prepared_queries
                .get("get_all_config")
                .expect("Query missing!"),
            ("",),
        )
        .await?
        .maybe_first_row_typed::<BasicThingDatabase>()?)
     */

    Ok(())
}
