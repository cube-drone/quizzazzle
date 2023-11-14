#[macro_use] extern crate rocket;
//mod config;
//mod scylla_up;
use std::env;
use std::sync::Arc;
use redis::cluster::ClusterClient;
use redis::AsyncCommands;
use rocket::{Rocket, Build};

mod error; // provides the no_shit! macro
mod basic;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct Services{
    pub cache_redis: Arc<ClusterClient>,
    pub application_redis: Arc<ClusterClient>,
    //pub scylla: Arc<scylla_up::Session>,
}

async fn setup_redis_cluster(redis_urls: &String) -> Arc<ClusterClient> {
    // Environment Variables
    let redis_nodes: Vec<&str> = redis_urls.split(",").collect();

    // Redis Setup
    let client = ClusterClient::new(redis_nodes).expect("Could not create Redis client");
    let mut connection = client.get_async_connection().await.expect("Could not connect to Redis");

    // Redis Test
    let _: () = connection.set("foo", "bar").await.expect("Could not test Redis connection");
    let result: String = connection.get("foo").await.expect("Could not test Redis connection");
    assert_eq!(result, "bar");

    return Arc::new(client);
}


#[launch]
async fn rocket() -> Rocket<Build> {

    // Environment Variables
    let cache_redis_urls = env::var("CACHE_REDIS_URLS").unwrap_or_else(|_| "".to_string());
    let application_redis_urls = env::var("APPLICATION_REDIS_URLS").unwrap_or_else(|_| "".to_string());

    // Service Setup
    let services = Services{
        cache_redis: setup_redis_cluster(&cache_redis_urls).await,
        application_redis: setup_redis_cluster(&application_redis_urls).await,
    };

    let mut app = rocket::build();

    app = app.manage(services);
    app = app.mount("/", routes![index]);
    app = basic::routes::mount_routes(app);

    app
}
