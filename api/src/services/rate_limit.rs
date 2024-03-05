use std::sync::Arc;
use std::time::Duration;
use moka::future::Cache;
use rocket::tokio;

pub struct RateLimitService{
    short_cache: Arc<Cache<String, bool>>,
    long_cache: Arc<Cache<String, usize>>,
}

#[derive(Clone)]
pub enum RateLimitType{
    Short,
    PerHour(usize),
    ShortAndPerHour(usize),
}

impl RateLimitService{
    pub fn new() -> Self {

        let short_cache: Cache<String, bool>;
        short_cache = Cache::builder()
            .max_capacity(100000)
            .time_to_live(Duration::from_secs(5))
            .build();

        let long_cache: Cache<String, usize>;
        let one_hour = 60*60;
        long_cache = Cache::builder()
            .max_capacity(100000)
            .time_to_live(Duration::from_secs(one_hour))
            .build();

        Self{
            short_cache: Arc::new(short_cache),
            long_cache: Arc::new(long_cache),
        }
    }

    pub async fn is_rate_limited(&self, prefix: &String, key: &String, limit_type: RateLimitType) -> bool {

        let key = format!("{}-{}", prefix, key);

        // there's a little gap between "get" and "insert" here that could be an issue, but 🤷
        let short = self.short_cache.get(&key);
        let long = self.long_cache.get(&key);

        let (short, long) = tokio::join!(short, long);

        match short {
            Some(shorts_fired) => {

                if shorts_fired{
                    match limit_type {
                        RateLimitType::Short => {
                            return true;
                        }
                        RateLimitType::ShortAndPerHour(_) => {
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }

        let mut long_val = 0;
        match long {
            Some(count) => {
                match limit_type {
                    RateLimitType::PerHour(max_attempts_per_hour) => {
                        if count >= max_attempts_per_hour {
                            return true;
                        }
                    }
                    RateLimitType::ShortAndPerHour(max_attempts_per_hour) => {
                        if count >= max_attempts_per_hour {
                            return true;
                        }
                    }
                    _ => {}
                }
                long_val = count;
            }
            None => {}
        }

        self.short_cache.insert(key.clone(), true).await;
        self.long_cache.insert(key.clone(), long_val+1).await;

        false
    }

    pub async fn is_any_rate_limited(&self, prefix: &String, keys: &Vec<String>, limit_type: RateLimitType) -> bool {
        let mut response = false;
        for key in keys {
            let is_rate_limited = self.is_rate_limited(prefix, key, limit_type.clone()).await;
            if is_rate_limited{
                response = true;
                // we can't return or continue here, because the "is_rate_limited" test also sets the rate limits for all the keys
            }
        }
        response
    }


}

#[tokio::test]
async fn limit_a_bunch_of_rates(){
    let service = RateLimitService::new();
    let prefix = "test".to_string();
    let key = "test".to_string();
    let rate_limit_type = RateLimitType::Short;

    // there's a 5-second rate limit: 1 request every 5 seconds
    let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, false);
    let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, true);

    let key = "toast".to_string();
    let rate_limit_type = RateLimitType::ShortAndPerHour(3);

    // there's a 5-second rate limit: 1 request every 5 seconds
    let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, false);
    let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, true);
}

#[tokio::test]
async fn limit_a_bunch_more_rates(){
    let service = RateLimitService::new();
    let prefix = "test".to_string();
    let key = "mest".to_string();
    let rate_limit_type = RateLimitType::PerHour(5);

    for _ in 0..5 {
        let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
        assert_eq!(is_rate_limited, false);
    }
    let is_rate_limited = service.is_rate_limited(&prefix, &key, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, true);
}

#[tokio::test]
async fn limit_a_list_of_rates(){
    let service = RateLimitService::new();
    let prefix = "test".to_string();
    let keys = vec!["moast".to_string(), "blurst".to_string()];
    let rate_limit_type = RateLimitType::PerHour(5);

    for _ in 0..5 {
        let is_rate_limited = service.is_any_rate_limited(&prefix, &keys, rate_limit_type.clone()).await;
        assert_eq!(is_rate_limited, false);
    }
    let is_rate_limited = service.is_any_rate_limited(&prefix, &keys, rate_limit_type.clone()).await;
    assert_eq!(is_rate_limited, true);
}