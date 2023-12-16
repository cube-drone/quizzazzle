#[macro_use]
extern crate rocket;

use std::fs;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
//use rocket::http::hyper::service;
use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket::tokio;
use redis::cluster::ClusterClient;
use redis::AsyncCommands;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::session::Session;
use scylla::SessionBuilder;
use comrak::{markdown_to_html, Options};

mod fairings;
mod error; // provides the no_shit! macro

mod config;
mod email;
mod basic;
mod home;
mod auth;

/*
    Services gets passed around willy nilly between threads so it needs to be cram-packed fulla arcs like a season of Naruto
*/
pub struct ScyllaService {
    pub session: Arc<Session>,
    pub prepared_queries: Arc<HashMap<&'static str, PreparedStatement>>,
}

/*
    Note that this is private and public in the "visible to the end-user" sense, not in the "OO" sense
*/
pub struct ConfigService {
    pub public_config: HashMap<String, String>,
    pub private_config: HashMap<String, String>
}

pub struct Services {
    pub is_production: bool,
    pub cache_redis: Arc<ClusterClient>,
    pub application_redis: Arc<ClusterClient>,
    pub scylla: ScyllaService,
    pub config: Arc<RwLock<ConfigService>>,
    pub email: Arc<email::EmailProvider>,
    pub static_markdown: Arc<HashMap<&'static str, String>>
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

//  "tos.md" -> look for /tmp/api-static/tos.md -> read it -> markdownify it -> return it
fn static_markdownify(file_name: &str) -> String {
    let file_path = format!("/tmp/api-static/{}", file_name);
    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let html = markdown_to_html(&file_contents, &Options::default());

    html
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // Environment Variables
    let is_production: bool = env::var("GROOVELET_PRODUCTION").unwrap_or_else(|_| "false".to_string()) == "true";
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
    let mut config_prepared_queries = config::model::initialize(&scylla_connection)
        .await
        .expect("Could not initialize config model");
    let mut other_prepared_queries: HashMap<&'static str, PreparedStatement> = HashMap::new();

    let queries_to_merge = vec![
        &mut basic_prepared_queries,
        &mut config_prepared_queries,
        &mut other_prepared_queries
    ];

    for query_map in queries_to_merge {
        prepared_queries.extend(query_map.drain());
    }

    // Static Content Setup
    let mut static_hashmap = HashMap::new();
    static_hashmap.insert("tos", static_markdownify("tos.md"));
    static_hashmap.insert("faq", static_markdownify("faq.md"));

    // Email Setup
    let email_provider = email::EmailProvider::setup().await;
    email_provider.send_hello("test@gooble.email".to_string()).await.expect("Could not send email");

    // Service Setup
    let services = Services {
        is_production: is_production,
        cache_redis: setup_redis_cluster(&cache_redis_urls).await,
        application_redis: setup_redis_cluster(&application_redis_urls).await,
        scylla: ScyllaService {
            session: scylla_connection,
            prepared_queries: Arc::new(prepared_queries),
        },
        config: Arc::new(RwLock::new(ConfigService{
            private_config: HashMap::new(),
            public_config: HashMap::new(),
        })),
        email: Arc::new(email_provider),
        static_markdown: Arc::new(static_hashmap),
    };

    let services_clone = Services{
        is_production: services.is_production,
        cache_redis: services.cache_redis.clone(),
        application_redis: services.application_redis.clone(),
        scylla: ScyllaService {
            session: services.scylla.session.clone(),
            prepared_queries: services.scylla.prepared_queries.clone()
        },
        config: services.config.clone(),
        email: services.email.clone(),
        static_markdown: services.static_markdown.clone()
    };

	// Launch App
    let mut app = rocket::build();

    app = app.manage(services);
    app = app.attach(crate::fairings::timing::RequestTimer)
             .attach(Template::fairing());

	// Mount Routes
    app = app.mount("/static", FileServer::from("/tmp/static"));
    app = app.mount("/build", FileServer::from("/tmp/build"));
    // home is where "/" lives.
    app = home::routes::mount_routes(app);
    // basic is a whole module intended to demonstrate basic functionality, it's not intended to be used in production
    app = basic::routes::mount_routes(app);
    // auth: login, registration, that sort of stuff
    app = auth::routes::mount_routes(app);
    // config: configuration
    app = config::routes::mount_routes(app);

    tokio::spawn(async move {
        loop{
            // code goes here
            println!("Every 5 seconds...");
            config::model::update_config(&services_clone).await.expect("Could not update config");

            // and now, I sleep
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    app
}
