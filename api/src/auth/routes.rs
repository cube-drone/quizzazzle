use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;

use crate::Services;

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

#[derive(FromForm)]
struct Invite<'r> {
    invite_code: &'r str,
}

//async fn counter(services: &State<Services>) -> Result<String, Status> {

#[post("/invite", data = "<invite>")]
async fn invite_post(_services: &State<Services>, invite: Form<Invite<'_>>) -> Result<Redirect, Template> {
    if invite.invite_code.len() > 10{
        return Err(Template::render("invite", context! {
            error: "Invite code too long",
        }));
    }
    println!("invite code: {}", invite.invite_code);

    //let uuid = services.get_invite_code_source(invite.invite_code).await;

    Ok(Redirect::to("/auth/ok"))
}

#[get("/ok")]
async fn ok() -> &'static str {
    "ok"
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/auth",
        routes![
            register,
            login,
            invite,
            invite_post,
            ok
        ],
    )
}
