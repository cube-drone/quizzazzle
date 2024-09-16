use crate::ministry_directory::{MinistryDirectory, DeckMetadata};
use anyhow::Result;
use crate::ministry_directory::Card;
use moka::future::Cache;

#[derive(Clone)]
struct CachedDeckMetadata {
    metadata: DeckMetadata,
    last_updated: std::time::SystemTime,
}

#[derive(Clone)]
struct CachedDeck {
    deck: Vec<Card>,
    last_updated: std::time::SystemTime,
}

#[derive(Clone)]
pub struct MinistryDirectoryCache {
    cached_metadata: Cache<String, CachedDeckMetadata>,
    cached_deck: Cache<String, CachedDeck>,
}

impl MinistryDirectoryCache {

    pub fn new() -> Self {
        MinistryDirectoryCache {
            cached_deck: Cache::new(2_000),
            cached_metadata: Cache::new(2_000),
        }
    }

    fn actually_get_metadata(&self, directory_root: &str) -> Result<DeckMetadata> {
        let ministry_directory = MinistryDirectory::new(directory_root.to_string());
        let metadata = ministry_directory.get_metadata()?;
        Ok(metadata)
    }

    pub async fn get_metadata(&self, directory_root: &str) -> Result<DeckMetadata> {

        let cached_metadata = self.cached_metadata.get(directory_root).await;
        let ministry_directory = MinistryDirectory::new(directory_root.to_string());
        let last_update_time = ministry_directory.get_last_update_time()?;

        if cached_metadata.is_some() {
            println!("Cache hit for {}: metadata", directory_root);
            // test if the cache is still valid
            let cached_metadata = cached_metadata.unwrap();
            // if the last update time is older than the cache, we can return it
            if last_update_time <= cached_metadata.last_updated {
                return Ok(cached_metadata.metadata);
            }
            else{
                println!("Cache is out of date for {}: metadata", directory_root);
            }
            // else: fall through and update the cache
        }
        println!("Cache miss for {}: metadata", directory_root);
        let actual_metadata = self.actually_get_metadata(directory_root)?;
        self.cached_metadata.insert(directory_root.to_string(), CachedDeckMetadata {
            metadata: actual_metadata.clone(),
            last_updated: last_update_time,
        }).await;

        Ok(actual_metadata)
    }

    fn actually_get_deck(&self, directory_root: &str) -> Result<Vec<Card>> {
        let ministry_directory = MinistryDirectory::new(directory_root.to_string());
        let deck = ministry_directory.get_deck()?;
        Ok(deck)
    }

    pub async fn get_deck(&self, directory_root: &str) -> Result<Vec<Card>> {

        let cached_deck = self.cached_deck.get(directory_root).await;
        let ministry_directory = MinistryDirectory::new(directory_root.to_string());
        let last_update_time = ministry_directory.get_last_update_time()?;

        if cached_deck.is_some() {
            println!("Cache hit for {}: deck", directory_root);
            // test if the cache is still valid
            let cached_deck = cached_deck.unwrap();
            // if the last update time is older than the cache, we can return it
            if last_update_time <= cached_deck.last_updated {
                return Ok(cached_deck.deck);
            }
            else{
                println!("Cache is out of date for {}: deck", directory_root);
            }
            // else: fall through and update the cache
        }
        println!("Cache miss for {}: deck", directory_root);
        let actual_deck = self.actually_get_deck(directory_root)?;
        self.cached_deck.insert(directory_root.to_string(), CachedDeck {
            deck: actual_deck.clone(),
            last_updated: last_update_time,
        }).await;

        Ok(actual_deck)
    }

}