use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};

#[get("/register")]
async fn register() -> &'static str {
    "Hello world"
}

/*
#[post("/register")]
async fn post_register() -> Redirect {
    // create an anon session
    // put the auth token in a cookie

}
 */

#[get("/login")]
async fn login() -> Template {
    Template::render("login", context! {
        foo: 123,
    })
}

#[get("/invite")]
async fn invite() -> Template {
    Template::render("invite", context! {})
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/auth",
        routes![
            register,
            login,
            invite,
        ],
    )
}
