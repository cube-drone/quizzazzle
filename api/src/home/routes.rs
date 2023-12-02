use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use chrono::Datelike;

#[get("/")]
async fn home() -> Template {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    Template::render("home", context! {
        year: year,
    })
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/",
        routes![
            home
        ],
    )
}
