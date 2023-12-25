use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket::serde::uuid::Uuid;
use rocket::http::CookieJar;

use crate::Services;
use crate::auth::model;

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

#[post("/invite", data = "<invite>")]
async fn invite_post(services: &State<Services>, cookies: &CookieJar<'_>, invite: Form<Invite<'_>>) -> Template {
    if invite.invite_code.len() < 1{
        return Template::render("invite", context! {
            error: "Invite code too short",
        });
    }
    if invite.invite_code.len() > 10{
        return Template::render("invite", context! {
            error: "Invite code too long",
        });
    }
    println!("invite code: {}", invite.invite_code);

    match services.get_invite_code_source(invite.invite_code).await{
        Ok(invite_source) => {
            println!("invite source: {}", invite_source);

            let csrf_token = Uuid::new_v4().to_string();
            cookies.add_private(("csrf_token", csrf_token.clone()));

            return Template::render("register", context! {
                csrf_token: csrf_token,
                invite_code: invite.invite_code,
            });
        },
        Err(e) => {
            return Template::render("invite", context! {
                error: e.to_string(),
            });
        }
    }

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
