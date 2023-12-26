use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket::serde::uuid::Uuid;
use rocket::http::CookieJar;

use validator::{Validate, ValidationError};

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

#[derive(FromForm, Validate)]
struct Invite<'r> {
    #[validate(length(min = 1, max = 10, message="Invite Code must be between 1 and 10 characters!"))]
    invite_code: &'r str,
}

#[post("/invite", data = "<invite>")]
async fn invite_post(services: &State<Services>, cookies: &CookieJar<'_>, invite: Form<Invite<'_>>) -> Template {
    match invite.validate() {
        Ok(_) => (),
        Err(e) => return Template::render("invite", context! {
            error: e.to_string(),
        }),
      };

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

#[derive(FromForm, Validate)]
struct Register<'r> {
    csrf_token: &'r str,
    invite_code: &'r str,
    #[validate(length(min = 3, max = 120, message="Display name must be between 3 and 120 characters!"))]
    display_name: &'r str,
    #[validate(email(message="Invalid email address!"))]
    email: &'r str,
    #[validate(length(min = 11, max = 300, message="Password must be between 11 and 300 characters!"))]
    password: &'r str,
    tos: bool,
    age: bool,
}

#[post("/register", data = "<register>")]
async fn register_post(services: &State<Services>, cookies: &CookieJar<'_>, register: Form<Register<'_>>) -> Template {

    let csrf_token_new = Uuid::new_v4().to_string();

    if let Some(csrf_cookie) = cookies.get_private("csrf_token"){
        let csrf_token_cookie = csrf_cookie.value();

        cookies.add_private(("csrf_token", csrf_token_new.clone()));
        if(register.csrf_token != csrf_token_cookie){
            return Template::render("register", context! {
                csrf_token: csrf_token_new,
                invite_code: register.invite_code,
                error: "CSRF token mismatch",
                display_name: register.display_name,
                email: register.email,
                password: register.password,
            })
        }
    }
    else{
        cookies.add_private(("csrf_token", csrf_token_new.clone()));

        return Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "CSRF cookie missing",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        })
    }

    if(!register.tos){
        return Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "You must agree to the terms of service",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        })
    }
    if(!register.age){
        return Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "You must be 13 years of age or older",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        })
    }
    match register.validate() {
        Ok(_) => (),
        Err(e) => return Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: e.to_string(),
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        }),
      };


    Template::render("register", context! {
        csrf_token: csrf_token_new,
        invite_code: register.invite_code,
        error: "Not implemented",
        display_name: register.display_name,
        email: register.email,
        password: register.password,
    })
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
            register_post,
            ok
        ],
    )
}
