use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket::serde::uuid::Uuid;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

use anyhow::anyhow;
use validator::Validate;

use crate::Services;
use crate::auth::model;

#[get("/login")]
async fn login() -> Template {
    Template::render("login", context! {
        foo: 123,
    })
}

#[get("/register")]
async fn register() -> Redirect  {
    /* since all registration requires an invite code, */
    Redirect::to("/auth/invite")
}

#[get("/invite")]
async fn invite() -> Template {
    Template::render("invite", context! {})
}

#[derive(FromForm, Validate)]
struct Invite<'r> {
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

    let invite_code = match model::InviteCode::from_string(invite.invite_code){
        Ok(invite_code) => invite_code,
        Err(e) => {
            return Template::render("invite", context! {
                error: e.to_string(),
            });
        }
    };

    match services.get_invite_code_source(&invite_code).await{
        Ok(invite_source) => {
            println!("invite source: {}", invite_source.to_string());

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
    invite_code: Uuid,
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
async fn register_post(services: &State<Services>, cookies: &CookieJar<'_>, register: Form<Register<'_>>) -> Result<Redirect, Template> {

    let csrf_token_new = Uuid::new_v4().to_string();

    if let Some(csrf_cookie) = cookies.get_private("csrf_token"){
        let csrf_token_cookie = csrf_cookie.value();

        cookies.add_private(("csrf_token", csrf_token_new.clone()));
        if register.csrf_token != csrf_token_cookie {
            return Err(Template::render("register", context! {
                csrf_token: csrf_token_new,
                invite_code: register.invite_code,
                error: "CSRF token mismatch",
                display_name: register.display_name,
                email: register.email,
                password: register.password,
            }))
        }
    }
    else{
        cookies.add_private(("csrf_token", csrf_token_new.clone()));

        return Err(Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "CSRF cookie missing",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        }))
    }

    if !register.tos {
        return Err(Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "You must agree to the terms of service",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        }))
    }
    if !register.age {
        return Err(Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "You must be 13 years of age or older",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        }))
    }
    match register.validate() {
        Ok(_) => (),
        Err(e) => return Err(Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: e.to_string(),
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        })),
      };

    // okay, now, let's try to create the user
    if let Ok(parent_uuid) = services.get_invite_code_source(&model::InviteCode::from_uuid(register.invite_code)).await{
        match services.exhaust_invite_code(&model::InviteCode::from_uuid(register.invite_code)).await{
            Ok(_) => (),
            Err(e) => {
                println!("Error exhausting invite code: {}", e);
                return Err(Template::render("register", context! {
                    csrf_token: csrf_token_new,
                    invite_code: register.invite_code,
                    error: "Error exhausting invite code",
                    display_name: register.display_name,
                    email: register.email,
                    password: register.password,
                }))
            }
        }

        let user_create = model::UserCreate{
            user_id: model::UserId::new(),
            display_name: register.display_name,
            email: register.email,
            parent_id: parent_uuid,
            password: register.password,
        };

        match services.create_user(user_create).await{
            Ok(session_token) => {
                // u did it, create a session token
                cookies.add_private(Cookie::new("session_token", session_token.to_string()));

                return Ok(Redirect::to("/auth/ok"))
            },
            Err(e) => {
                println!("Error creating user: {}", e);
                return Err(Template::render("register", context! {
                    csrf_token: csrf_token_new,
                    invite_code: register.invite_code,
                    error: "Error creating user",
                    display_name: register.display_name,
                    email: register.email,
                    password: register.password,
                }))
            }
        }
    }
    else{
        return Err(Template::render("register", context! {
            csrf_token: csrf_token_new,
            invite_code: register.invite_code,
            error: "Invalid invite code",
            display_name: register.display_name,
            email: register.email,
            password: register.password,
        }))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for model::UserSession {

    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, anyhow::Error> {
        let services = req.rocket().state::<Services>().unwrap();

        let maybe_session_token = req.cookies().get_private("session_token");

        if let Some(session_token) = maybe_session_token{
            let session_token_maybe = model::SessionToken::from_string(session_token.value());

            match session_token_maybe{
                Ok(session_token) => {
                    match services.get_user_from_session_token(&session_token).await{
                        Ok(user) => {
                            return Outcome::Success(user);
                        },
                        Err(e) => {
                            println!("Error getting user from session token: {}", e);
                            return Outcome::Error((Status::Unauthorized, anyhow!("Error getting user from session token")));
                        }
                    }
                },
                Err(_) => {
                    return Outcome::Error((Status::Unauthorized, anyhow!("Invalid session token")));
                }
            }
        }
        else{
            return Outcome::Error((Status::Unauthorized, anyhow!("No session token")));
        }
    }
}

#[get("/ok")]
async fn ok_user(user: model::UserSession) -> &'static str {
    "ok, user"
}

#[get("/ok", rank=2)]
async fn ok() -> &'static str {
    "ok"
}

#[get("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::from("session_token"));

    Redirect::to("/")
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
            ok_user,
            ok,
            logout
        ],
    )
}
