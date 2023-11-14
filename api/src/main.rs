#[macro_use] extern crate rocket;
//mod config;
//mod scylla_up;
use std::env;
use std::sync::Arc;
use redis::cluster::ClusterClient;
use redis::AsyncCommands;
use rocket::State;
use rocket::http::Status;

fn error_response(error_string: String) -> Status {
    eprintln!("ERROR!: {}", error_string);
    return Status::InternalServerError;
}

macro_rules! no_shit {
    ($message:expr) => {
        $message.map_err(|err| error_response(err.to_string()))?
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/counter")]
async fn counter(services: &State<Services>) -> Result<String, Status> {
    let mut redis_connection = no_shit!( services.cache_redis.get_async_connection().await );
    let counter_result:i64 = no_shit!( redis_connection.incr("counter", 1).await );

    Ok(format!("Counter: {counter_result}"))
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
async fn rocket() -> _ {

    // Environment Variables
    let cache_redis_urls = env::var("CACHE_REDIS_URLS").unwrap_or_else(|_| "".to_string());
    let application_redis_urls = env::var("APPLICATION_REDIS_URLS").unwrap_or_else(|_| "".to_string());

    // Service Setup
    let services = Services{
        cache_redis: setup_redis_cluster(&cache_redis_urls).await,
        application_redis: setup_redis_cluster(&application_redis_urls).await,
    };

    rocket::build().manage(services)
        .mount("/", routes![index, counter])
}
