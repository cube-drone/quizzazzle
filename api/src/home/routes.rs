use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{Template, context};
use chrono::Datelike;

use crate::Services;

#[get("/")]
async fn home() -> Template {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    Template::render("home", context! {
        year: year,
    })
}

#[get("/tos")]
async fn tos(services: &State<Services>) -> Template {
    Template::render("markdown", context!{
        title: "ToS",
        content: services.static_markdown.get("tos").expect("tos should be available")
    })
}

#[get("/faq")]
async fn faq(services: &State<Services>) -> Template {
    Template::render("markdown", context!{
        title: "FAQ",
        content: services.static_markdown.get("faq").expect("faq should be available")
    })
}

#[get("/home")]
async fn user_home(_user: crate::auth::model::VerifiedUserSession) -> Template {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    Template::render("user_home", context! {
        year: year,
    })
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/",
        routes![
            home,
            tos,
            faq,
            user_home
        ],
    )
}
