use anyhow::Result;
use rocket::serde::uuid::Uuid;
use chrono::Utc;

use crate::basic::types::{BasicThingCreate, BasicThingDatabase, BasicThingPublic};
use crate::Services;

pub fn sanitize_basic_thing(basic_thing: BasicThingCreate) -> BasicThingDatabase {
    BasicThingDatabase {
        id: Uuid::new_v4(),
        name: basic_thing.name,
        created_at: chrono::Duration::milliseconds(Utc::now().timestamp_millis()),
    }
}

pub fn transform_basic_thing(basic_thing: BasicThingDatabase) -> BasicThingPublic {
    BasicThingPublic {
        id: basic_thing.id,
        name: basic_thing.name,
        created_at: basic_thing.created_at.num_milliseconds(),
    }
}

pub async fn create_basic_thing(
    services: &Services,
    basic_thing: BasicThingCreate,
) -> Result<BasicThingPublic> {
    let basic_thing_created = sanitize_basic_thing(basic_thing);

    crate::basic::model::create_basic_thing(&services.scylla, &basic_thing_created).await?;

    Ok(transform_basic_thing(basic_thing_created))
}

pub async fn get_basic_thing(services: &Services, uuid: &Uuid) -> Result<Option<BasicThingPublic>> {
    let basic_thing = crate::basic::model::get_basic(&services.scylla, uuid).await?;

    match basic_thing {
        Some(basic_thing) => Ok(Some(transform_basic_thing(basic_thing))),
        None => Ok(None),
    }
}
