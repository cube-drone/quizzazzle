use anyhow::Result;
use rocket::serde::uuid::Uuid;

use crate::Services;

const TEST_INDEX_ID: IndexId = IndexId(Uuid::from_u128(0));

pub struct IndexId(Uuid);
impl IndexId {
    pub fn new() -> Self {
        IndexId(Uuid::new_v4())
    }
    pub fn from_uuid(user_id: Uuid) -> Self {
        IndexId(user_id)
    }
    pub fn from_string(user_id: &str) -> Result<Self> {
        Ok(IndexId(Uuid::parse_str(user_id)?))
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

pub struct ContentId(String);
impl ContentId {
    pub fn new() -> Self {
        ContentId("test".to_string())
    }
    pub fn from_string(content_id: &str) -> Self {
        ContentId(content_id.to_string())
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct Index{
    pub id: IndexId,
    pub user_slug: String,
    pub content_slug: String,
    pub name: String,
    pub description: String,
    pub thumbnail_image_url: String,
    pub order: String,
    pub content_ids: Vec<ContentId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub enum Content {
    Markdown{
        content: String
    },
    Image{
        url: String
    },
}


pub struct ContentNode{
    pub id: ContentId,
    pub order: usize,
    pub content: Content,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Services{

    pub async fn get_index_id(&self, user_slug: &str, feed_slug: &str) -> Result<Option<IndexId>> {
        // take userSlug and indexSlug, => indexId
        // take contentId (if no contentId, use the first one)

        if(user_slug == "test" && feed_slug == "test"){
            return Ok(Some(TEST_INDEX_ID));
        }

        Ok(None)
    }

    pub async fn get_index(&self, index_id: &IndexId) -> Result<Option<Index>> {
        if index_id.to_string() == TEST_INDEX_ID.to_string() {
            return Ok(Some(Index{
                id: TEST_INDEX_ID,
                user_slug: "test".to_string(),
                content_slug: "test".to_string(),
                name: "test".to_string(),
                description: "test".to_string(),
                thumbnail_image_url: "test".to_string(),
                order: "test".to_string(),
                content_ids: vec![ContentId::new()],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }));
        }

        Ok(None)
    }

    pub async fn get_content(&self, index_id: &IndexId, _content_id: &ContentId) -> Result<Option<ContentNode>> {
        if index_id.to_string() == TEST_INDEX_ID.to_string() {
            return Ok(Some(ContentNode{
                id: ContentId::new(),
                order: 0,
                content: Content::Markdown{
                    content: "test".to_string()
                },
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }));
        }

        Ok(None)
    }

}