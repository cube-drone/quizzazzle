pub use scylla::transport::session::{IntoTypedRows, Session};
use scylla::SessionBuilder;
use anyhow::{Result, Error};
use crate::config;

pub async fn scylla_setup(config: &config::Config) -> Result<Session, Error> {
    let uri = config.scylla_url.clone();
    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await?;

    return Ok(session);
}