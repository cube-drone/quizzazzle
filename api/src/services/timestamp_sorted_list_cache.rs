use std::sync::Arc;
use std::time::Duration;
use std::collections::VecDeque;
use rocket::tokio::sync::RwLock;
use anyhow::{Result, anyhow};

use rocket::tokio;
use rocket::serde::uuid::Uuid;

use moka::future::Cache;


pub struct TimestampSortedListCache<T: 'static> where T: Clone + Send + Sync + PartialEq{
    cache: Cache<Uuid, Arc<RwLock<VecDeque<(T, i64)>>>>,
    vec_capacity: usize,
}

impl<T> TimestampSortedListCache<T> where T: Clone + Send + Sync + PartialEq{
    pub fn new(cache_capacity: u64, expiry_seconds: u64, vec_capacity: usize) -> TimestampSortedListCache<T>{
        let cache = Cache::builder()
            .max_capacity(cache_capacity)
            .time_to_idle(Duration::from_secs(expiry_seconds))
            .build();
        TimestampSortedListCache{cache, vec_capacity}
    }

    pub async fn get(&self, key: &Uuid) -> Option<VecDeque<(T, i64)>> {
        let mutex = self.cache.get(&key).await;
        match mutex {
            Some(arc) => {
                let readable = arc.read().await;
                Some(readable.clone())
            }
            None => None,
        }
    }

    pub async fn clear(&self, key: &Uuid) {
        self.cache.remove(&key).await;
    }

    pub async fn create_empty(&self, key: &Uuid) -> Result<()> {
        let arc = Arc::new(RwLock::new(VecDeque::with_capacity(self.vec_capacity)));
        self.cache.insert(key.clone(), arc).await;
        Ok(())
    }

    pub async fn load(&self, key: &Uuid, value: VecDeque<(T, i64)>) -> Result<()> {

        // make sure that value is sorted - we will keep it sorted in place,
        //  but if it is not sorted to begin with, we will have a bad time
        let mut value = value;
        value.make_contiguous().sort_by(|a, b| b.1.cmp(&a.1));

        let arc = Arc::new(RwLock::new(value));

        self.cache.insert(key.clone(), arc).await;
        Ok(())
    }

    pub fn exists(&self, key: &Uuid) -> bool {
        self.cache.contains_key(&key)
    }

    pub async fn count(&self, key: &Uuid) -> usize {
        match self.cache.get(&key).await {
            Some(arc) => {
                let readable = arc.read().await;
                readable.len()
            }
            None => 0,
        }
    }

    pub async fn pop_oldest(&self, key: &Uuid) -> Option<T> {
        let arc = match self.cache.get(&key).await {
            Some(arc) => arc,
            None => return None,
        };
        let mut writable = arc.write().await;

        match writable.pop_back() {
            Some((token, _)) => Some(token),
            None => None,
        }
    }

    pub async fn push_new(&self, key: &Uuid, thing_to_push: T) -> Result<()> {
        let arc = match self.cache.get(&key).await {
            Some(arc) => arc,
            None => return Err(anyhow!("that key doesn't exist")),
        };
        let mut writable = arc.write().await;

        let mut index_to_remove = -1;
        for (i, (token, _)) in writable.iter().enumerate() {
            if *token == thing_to_push {
                index_to_remove = i as isize;
                break;
            }
        }
        if index_to_remove >= 0 {
            writable.remove(index_to_remove as usize);
        }

        writable.push_front((thing_to_push, chrono::Utc::now().timestamp()));

        Ok(())
    }
}


#[tokio::test]
async fn test_timesorted_list_cache(){
    let cache_capacity = 10;
    let expiry_seconds = 10;
    let vec_capacity = 10;
    let cache: TimestampSortedListCache<Uuid> = TimestampSortedListCache::new(cache_capacity, expiry_seconds, vec_capacity);

    let user_id = Uuid::new_v4();
    cache.create_empty(&user_id).await.unwrap();

    let oldest = Uuid::new_v4();
    cache.push_new(&user_id, oldest).await.unwrap();
    let middlest = Uuid::new_v4();
    cache.push_new(&user_id, middlest).await.unwrap();
    let newest = Uuid::new_v4();
    cache.push_new(&user_id, newest).await.unwrap();

    let count = cache.count(&user_id).await;
    assert_eq!(count, 3);

    let got = cache.get(&user_id).await.unwrap();
    assert_eq!(got.len(), 3);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, oldest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 2);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, middlest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 1);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, newest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_update(){
    let cache_capacity = 10;
    let expiry_seconds = 10;
    let vec_capacity = 10;
    let cache: TimestampSortedListCache<Uuid> = TimestampSortedListCache::new(cache_capacity, expiry_seconds, vec_capacity);

    let user_id = Uuid::new_v4();
    cache.create_empty(&user_id).await.unwrap();

    let oldest = Uuid::new_v4();
    cache.push_new(&user_id, oldest).await.unwrap();
    let middlest = Uuid::new_v4();
    cache.push_new(&user_id, middlest).await.unwrap();
    let newest = Uuid::new_v4();
    cache.push_new(&user_id, newest).await.unwrap();

    let count = cache.count(&user_id).await;
    assert_eq!(count, 3);

    // this takes the middlest and makes it the newest
    cache.push_new(&user_id, middlest).await.unwrap();

    // which shouldn't create a new entry
    let count = cache.count(&user_id).await;
    assert_eq!(count, 3);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, oldest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 2);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, newest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 1);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, middlest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_load_sorting(){
    let cache_capacity = 10;
    let expiry_seconds = 10;
    let vec_capacity = 10;
    let cache: TimestampSortedListCache<Uuid> = TimestampSortedListCache::new(cache_capacity, expiry_seconds, vec_capacity);

    let oldest = Uuid::new_v4();
    let middlest = Uuid::new_v4();
    let newest = Uuid::new_v4();

    let list = vec![
        (middlest, 2),
        (oldest, 1),
        (newest, 3),
    ];

    let user_id = Uuid::new_v4();
    cache.load(&user_id, VecDeque::from(list)).await.unwrap();

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, oldest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 2);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, middlest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 1);

    let popped = cache.pop_oldest(&user_id).await.unwrap();
    assert_eq!(popped, newest);

    let count = cache.count(&user_id).await;
    assert_eq!(count, 0);

}

#[tokio::test]
async fn test_clear(){
    let cache_capacity = 10;
    let expiry_seconds = 10;
    let vec_capacity = 10;
    let cache: TimestampSortedListCache<Uuid> = TimestampSortedListCache::new(cache_capacity, expiry_seconds, vec_capacity);

    let user_id = Uuid::new_v4();
    cache.create_empty(&user_id).await.unwrap();

    let oldest = Uuid::new_v4();
    cache.push_new(&user_id, oldest).await.unwrap();
    let middlest = Uuid::new_v4();
    cache.push_new(&user_id, middlest).await.unwrap();
    let newest = Uuid::new_v4();
    cache.push_new(&user_id, newest).await.unwrap();

    let count = cache.count(&user_id).await;
    assert_eq!(count, 3);

    cache.clear(&user_id).await;

    let count = cache.count(&user_id).await;
    assert_eq!(count, 0);
}