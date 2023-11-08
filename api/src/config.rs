use std::env;

pub struct Config {
    pub scylla_url: String,
    pub auth_redis_url: String,
    pub cache_redis_url: String,
    pub search_redis_url: String,
    pub message_redis_url: String,
}

pub fn config() -> Config {
    let scylla_url = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let auth_redis_url = env::var("AUTH_REDIS_URI").unwrap_or_else(|_| "127.0.0.1:6032".to_string());
    let cache_redis_url = env::var("CACHE_REDIS_URI").unwrap_or_else(|_| "127.0.0.1:6033".to_string());
    let search_redis_url = env::var("SEARCH_REDIS_URI").unwrap_or_else(|_| "127.0.0.1:6034".to_string());
    let message_redis_url = env::var("MESSAGE_REDIS_URI").unwrap_or_else(|_| "127.0.0.1:6035".to_string());

    let conf = Config {
        scylla_url: scylla_url,
        auth_redis_url: auth_redis_url,
        cache_redis_url: cache_redis_url,
        search_redis_url: search_redis_url,
        message_redis_url: message_redis_url,
    };
    return conf;
}