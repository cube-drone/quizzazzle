use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;

#[get("/feed")]
async fn feeds() -> Template {
    // take userSlug and indexSlug, => indexId
    // take contentId (if no contentId, use the first one)
    Template::render("feed", context! {
    })
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/feed",
        routes![
            feeds
        ],
    )
}
