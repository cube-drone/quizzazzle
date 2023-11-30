#[macro_use]
extern crate rocket;
use redis::cluster::ClusterClient;
use redis::AsyncCommands;
use rocket::{Build, Rocket};
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::session::Session;
use scylla::SessionBuilder;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

mod basic;
mod fairings;
mod error; // provides the no_shit! macro

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
    Services gets passed around willy nilly between threads so it needs to be cram-packed fulla arcs like a season of Naruto
*/
pub struct ScyllaService {
    pub session: Arc<Session>,
    pub prepared_queries: Arc<HashMap<&'static str, PreparedStatement>>,
}

pub struct Services {
    pub cache_redis: Arc<ClusterClient>,
    pub application_redis: Arc<ClusterClient>,
    pub scylla: ScyllaService,
}

async fn setup_redis_cluster(redis_urls: &String) -> Arc<ClusterClient> {
    // Environment Variables
    let redis_nodes: Vec<&str> = redis_urls.split(",").collect();

    // Redis Setup
    let client = ClusterClient::new(redis_nodes).expect("Could not create Redis client");
    let mut connection = client
        .get_async_connection()
        .await
        .expect("Could not connect to Redis");

    // Redis Test
    let _: () = connection
        .set("foo", "bar")
        .await
        .expect("Could not test Redis connection");
    let result: String = connection
        .get("foo")
        .await
        .expect("Could not test Redis connection");
    assert_eq!(result, "bar");

    Arc::new(client)
}

async fn setup_scylla_cluster(scylla_url: &String) -> Arc<Session> {
    let session: Session = SessionBuilder::new()
        .known_node(scylla_url)
        .build()
        .await
        .expect("Could not connect to Scylla");

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 2}", &[]).await.expect("Could not create keyspace");

    Arc::new(session)
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // Environment Variables
    let cache_redis_urls = env::var("CACHE_REDIS_URLS").unwrap_or_else(|_| "".to_string());
    let application_redis_urls =
        env::var("APPLICATION_REDIS_URLS").unwrap_or_else(|_| "".to_string());

    // Scylla Setup
    let scylla_url = env::var("SCYLLA_URL").unwrap_or_else(|_| "".to_string());
    let scylla_connection = setup_scylla_cluster(&scylla_url).await;
    let mut prepared_queries: HashMap<&'static str, PreparedStatement> = HashMap::new();

    // Initialize Models & Prepare all Scylla Queries
    let mut basic_prepared_queries = basic::model::initialize(&scylla_connection)
        .await
        .expect("Could not initialize basic model");
    let mut other_prepared_queries: HashMap<&'static str, PreparedStatement> = HashMap::new();
    let queries_to_merge = vec![&mut basic_prepared_queries, &mut other_prepared_queries];

    for query_map in queries_to_merge {
        prepared_queries.extend(query_map.drain());
    }

    // Service Setup
    let services = Services {
        cache_redis: setup_redis_cluster(&cache_redis_urls).await,
        application_redis: setup_redis_cluster(&application_redis_urls).await,
        scylla: ScyllaService {
            session: scylla_connection,
            prepared_queries: Arc::new(prepared_queries),
        },
    };

    let mut app = rocket::build();

    app = app.manage(services);
    app = app.mount("/", routes![index]);
    app = app.attach(crate::fairings::timing::RequestTimer);
    app = basic::routes::mount_routes(app);

    app
}
