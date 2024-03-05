use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
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

#[get("/rate")]
async fn rate() -> Template {
    Template::render("message", context! {
        title: "Rate Limited",
        message: "You're doing that too much. Please wait a bit before trying again."
    })
}

#[post("/rate")]
async fn rate_post() -> Template {
    Template::render("message", context! {
        title: "Rate Limited",
        message: "You're doing that too much. Please wait a bit before trying again."
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

#[get("/pricing")]
async fn pricing(services: &State<Services>) -> Template {
    Template::render("markdown", context!{
        title: "Pricing",
        content: services.static_markdown.get("pricing").expect("pricing should be available")
    })
}

#[get("/home")]
async fn user_home(user: crate::auth::model::VerifiedUserSession) -> Template {
    let display_name = user.display_name;
    Template::render("user_home", context! {
        display_name: display_name,
    })
}

#[get("/home", rank = 2)]
async fn user_home_bounce() -> Redirect {
    Redirect::to("/auth/login")
}


pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/",
        routes![
            home,
            rate,
            rate_post,
            tos,
            faq,
            pricing,
            user_home,
            user_home_bounce
        ],
    )
}
