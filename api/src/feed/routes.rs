use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{Template, context};
use rocket::http::Status;

use crate::Services;

#[get("/<user_slug>/<feed_slug>")]
async fn feeds(services: &State<Services>, user_slug: &str, feed_slug: &str) -> Result<Template, Status> {
    // take userSlug and indexSlug, => indexId
    // take contentId (if no contentId, use the first one)

    // if the indexId can't be found, return 404
    let index_id = services.get_index_id(user_slug, feed_slug).await.map_err(|_| Status::NotFound)?.ok_or(Status::NotFound)?;

    let index = services.get_index(&index_id).await.map_err(|_| Status::NotFound)?.ok_or(Status::NotFound)?;

    Ok(Template::render("feed", context! {
        index_id: index_id.to_string(),
        user_slug: user_slug.to_string(),
        content_slug: feed_slug.to_string(),
        name: index.name,
        description: index.description,
        thumbnail_image_url: index.thumbnail_image_url,
        order: index.order,
    }))
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/feed",
        routes![
            feeds
        ],
    )
}
