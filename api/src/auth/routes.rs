use std::collections::HashMap;
use std::net::IpAddr;

use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket::serde::uuid::Uuid;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use rocket::serde::json::Json;

use validator::Validate;

use crate::Services;
use crate::auth::model;

#[get("/login")]
async fn login(cookies: &CookieJar<'_>) -> Template {

    let csrf_token = Uuid::new_v4().to_string();
    cookies.add_private(("csrf_token", csrf_token.clone()));

    Template::render("login", context! {
        csrf_token: csrf_token,
    })
}

#[derive(FromForm, Validate)]
struct Login<'r> {
    csrf_token: &'r str,
    #[validate(email(message="Invalid email address!"))]
    email: &'r str,
    password: &'r str,
}

const MAXIMUM_LOGIN_ATTEMPTS_PER_HOUR: usize = 15;

#[post("/login", data = "<login>")]
async fn login_post(services: &State<Services>, cookies: &CookieJar<'_>, login: Form<Login<'_>>, ip: BestGuessIpAddress) -> Result<Redirect, Template> {

    let csrf_token_new = Uuid::new_v4().to_string();

    if let Some(csrf_cookie) = cookies.get_private("csrf_token"){
        let csrf_token_cookie = csrf_cookie.value();

        cookies.add_private(("csrf_token", csrf_token_new.clone()));
        if login.csrf_token != csrf_token_cookie {
            return Err(Template::render("login", context! {
                csrf_token: csrf_token_new,
                error: "CSRF token mismatch",
                email: login.email,
                password: login.password,
            }));
        }
    }
    else{
        cookies.add_private(("csrf_token", csrf_token_new.clone()));
        return Err(Template::render("login", context! {
            csrf_token: csrf_token_new,
            error: "CSRF token missing",
            email: login.email,
            password: login.password,
        }));
    }

    match login.validate() {
        Ok(_) => (),
        Err(e) => return Err(Template::render("login", context! {
            csrf_token: csrf_token_new,
            error: e.to_string(),
            email: login.email,
            password: login.password,
        })),
      };

    let rate_limit_factors: Vec<String> = vec![ip.to_string(), login.email.to_string()];
    match services.rate_limits(&rate_limit_factors, MAXIMUM_LOGIN_ATTEMPTS_PER_HOUR).await{
        Ok(()) => {
        },
        Err(_e) => {
            return Err(Template::render("login", context! {
                csrf_token: csrf_token_new,
                error: "Attempting logins too fast, please wait a bit and try again!",
                email: "",
                password: "",
            }));
        }
    }

    // okay, now, let's try to login
    match services.login(login.email, login.password, ip.to_ip()).await{
        Ok(session_token) => {
            // u did it, create a session token
            cookies.add_private(Cookie::new("session_token", session_token.to_string()));

            Ok(Redirect::to("/auth/ok"))
        },
        Err(e) => {
            println!("Error logging in: {}", e);
            Err(Template::render("login", context! {
                csrf_token: csrf_token_new,
                error: "Could not log in",
                email: login.email,
                password: login.password,
            }))
        }
    }
}

#[get("/register")]
async fn register() -> Redirect  {
    /* since all registration requires an invite code, */
    Redirect::to("/auth/invite")
}

#[get("/test/generate_invite_code")]
async fn test_generate_invite_code(services: &State<Services>) -> Result<Json<HashMap<String, String>>, Status> {
    if services.is_production {
        return Err(Status::Forbidden);
    }

    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("invite_code".to_string(),
        services.generate_invite_code().await.expect("should be able to generate an invite code").to_string()
    );

    Ok(Json(hashmap))
}

#[post("/test/create_user", format = "json", data = "<user_serialized>")]
async fn test_create_user(services: &State<Services>, cookies: &CookieJar<'_>, ip: BestGuessIpAddress, user_serialized: Json<model::UserCreate<'_>>) -> Result<Json<HashMap<String, String>>, Status> {
    if services.is_production {
        return Err(Status::Forbidden);
    }

    let user_to_create = user_serialized.into_inner();
    let user_id = user_to_create.user_id.clone();

    let user_to_create = model::UserCreate{
        ip: ip.to_ip(),
        ..user_to_create
    };

    let session_token = services.create_user(user_to_create).await.expect("should be able to create a user");
    cookies.add_private(Cookie::new("session_token", session_token.to_string()));

    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("session_token".to_string(), session_token.to_string());
    hashmap.insert("user_id".to_string(), user_id.to_string());

    Ok(Json(hashmap))
}

#[get("/test/forget_ip")]
async fn test_forget_ip(service: &State<Services>, cookies: &CookieJar<'_>, ip: BestGuessIpAddress, user: model::VerifiedUserSession) -> Result<Redirect, Status> {
    if service.is_production {
        return Err(Status::Forbidden);
    }

    service.forget_ip(&user.user_id, &ip.to_ip()).await.expect("should be able to forget ip");

    cookies.remove_private(Cookie::from("session_token"));

    Ok(Redirect::to("/"))
}


#[get("/test/get_last_email?<email>")]
async fn test_get_last_email(services: &State<Services>, email: &str) -> Result<Json<HashMap<String, String>>, Status> {
    if services.is_production {
        return Err(Status::Forbidden);
    }

    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("email".to_string(),
        services.test_get_last_email(&email).await.expect("should be able to get last email").to_string()
    );

    Ok(Json(hashmap))
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
async fn register_post(services: &State<Services>, cookies: &CookieJar<'_>, ip: BestGuessIpAddress, register: Form<Register<'_>>) -> Result<Redirect, Template> {

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
            is_verified: false,
            is_admin: false,
            ip: ip.to_ip(),
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
impl<'r> FromRequest<'r> for model::AdminUserSession {

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
                            if user.is_admin{
                                return Outcome::Success(user.to_admin_user_session());
                            }
                            else{
                                return Outcome::Forward(Status::Forbidden);
                            }
                        },
                        Err(e) => {
                            println!("Error getting user from session token: {}", e);
                            return Outcome::Forward(Status::Forbidden);
                        }
                    }
                },
                Err(_) => {
                    return Outcome::Forward(Status::Forbidden);
                }
            }
        }
        else{
            return Outcome::Forward(Status::Forbidden);
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for model::VerifiedUserSession {

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
                            if user.is_verified && user.is_known_ip{
                                return Outcome::Success(user.to_verified_user_session());
                            }
                            else{
                                return Outcome::Forward(Status::Forbidden);
                            }
                        },
                        Err(e) => {
                            println!("Error getting user from session token: {}", e);
                            return Outcome::Forward(Status::Forbidden);
                        }
                    }
                },
                Err(_) => {
                    return Outcome::Forward(Status::Forbidden);
                }
            }
        }
        else{
            return Outcome::Forward(Status::Forbidden);
        }
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
                            return Outcome::Forward(Status::Forbidden);
                        }
                    }
                },
                Err(_) => {
                    return Outcome::Forward(Status::Forbidden);
                }
            }
        }
        else{
            return Outcome::Forward(Status::Forbidden);
        }
    }
}

struct BestGuessIpAddress(IpAddr);
impl BestGuessIpAddress{
    fn from_string(ip: &str) -> Result<Self, anyhow::Error>{
        let ip = ip.parse::<IpAddr>()?;
        Ok(Self(ip))
    }
    fn from_ip(ip: IpAddr) -> Self{
        Self(ip)
    }
    fn to_string(&self) -> String{
        self.0.to_string()
    }
    fn to_ip(&self) -> IpAddr{
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BestGuessIpAddress {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, anyhow::Error> {
        // X-Forwarded-For is a comma-separated list of IPs, the first one is the client IP
        // return that, or, failing that
        let maybe_ip = req.headers().get_one("X-Forwarded-For");
        match maybe_ip{
            Some(ip) => {
                let ip = ip.split(",").next().unwrap().to_string();
                match BestGuessIpAddress::from_string(&ip){
                    Ok(ip) => return Outcome::Success(ip),
                    Err(e) => {
                        println!("Error parsing ip address: {}", e);
                        return Outcome::Error((Status::BadRequest, anyhow::anyhow!("error parsing ip address")));
                    }
                }
            },
            None => {
                let maybe_ip = req.remote().map(|ip| ip.ip());
                match maybe_ip{
                    Some(ip) => return Outcome::Success(BestGuessIpAddress::from_ip(ip)),
                    None => return Outcome::Error((Status::BadRequest, anyhow::anyhow!("error parsing ip address"))),
                }
            }
        }
    }
}

#[get("/verify_email", rank=1)]
async fn verify_email_ok(_user: model::VerifiedUserSession) -> Redirect{
    /* if the user is already verified, no need to show them anything, move them along */
    Redirect::to("/auth/ok")
}

#[get("/verify_email", rank=3)]
async fn verify_email_template(_user: model::UserSession) -> Template{
    Template::render("verify_email", context! {})
}

#[get("/verify_email?<token>", rank=2)]
async fn verify_email(services: &State<Services>, token: Uuid) -> Redirect {
    let maybe_error = services.verify_email(&token).await;

    match maybe_error{
        Ok(_) => Redirect::to("/auth/ok"),
        Err(e) => {
            println!("Error verifying email: {}", e);
            Redirect::to("/auth/email_error")
        }
    }
}

#[get("/verify_email", rank=4)]
async fn verify_email_nobody() -> Redirect{
    /* if the user is already verified, no need to show them anything, move them along */
    Redirect::to("/auth/login")
}

#[get("/verify_ip", rank=1)]
async fn verify_ip_ok(_user: model::VerifiedUserSession) -> Redirect{
    /* if the user is already verified, no need to show them anything, move them along */
    Redirect::to("/auth/ok")
}

#[get("/verify_ip", rank=3)]
async fn verify_ip_template(_user: model::UserSession) -> Template{
    Template::render("verify_ip", context! {})
}

#[get("/verify_ip?<token>", rank=2)]
async fn verify_ip(services: &State<Services>, token: Uuid, ip: BestGuessIpAddress) -> Redirect {
    let maybe_error = services.verify_ip(&token, &ip.to_ip()).await;

    match maybe_error{
        Ok(_) => Redirect::to("/auth/ok"),
        Err(e) => {
            println!("Error verifying ip: {}", e);
            Redirect::to("/auth/ip_error")
        }
    }
}

#[get("/verify_ip", rank=4)]
async fn verify_ip_nobody() -> Redirect{
    /* if the user is already verified, no need to show them anything, move them along */
    Redirect::to("/auth/login")
}


#[get("/status")]
async fn status_auth_user(_admin: model::AdminUserSession) -> &'static str {
    "ok, auth user"
}

#[get("/status", rank=2)]
async fn status_verified_user(_user: model::VerifiedUserSession) -> &'static str {
    "ok, verified user"
}

#[get("/status", rank=3)]
async fn status_user(_user: model::UserSession) -> &'static str {
    "ok, user"
}

#[get("/status", rank=4)]
async fn status() -> &'static str {
    "ok, not logged in"
}

#[get("/email_error")]
async fn email_error() -> Template {
    Template::render("error", context! {error: "We tried to verify your email, but something went wrong. Please try again!"})
}

#[get("/ip_error")]
async fn ip_error() -> Template {
    Template::render("error", context! {error: "We tried to verify your location, but something went wrong. Please try again!"})
}

#[get("/ok")]
async fn ok_verified_user(_user: model::VerifiedUserSession) -> Redirect {
    Redirect::to("/home")
}

#[get("/ok", rank=2)]
async fn ok_user(user: model::UserSession) -> Redirect {
    if !user.is_verified {
        return Redirect::to("/auth/verify_email");
    }
    if !user.is_known_ip {
        return Redirect::to("/auth/verify_ip");
    }
    // shouldn't be able to get here
    Redirect::to("/home")
}

#[get("/ok", rank=3)]
async fn ok() -> Redirect {
    Redirect::to("/auth/login")
}

#[get("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Redirect {

    cookies.remove_private(Cookie::from("session_token"));

    Redirect::to("/")
}

#[get("/user")]
async fn auth_user(user: model::VerifiedUserSession) -> Json<model::VerifiedUserSession> {
    Json(user)
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/auth",
        routes![
            login,
            login_post,
            register,
            test_generate_invite_code,
            test_create_user,
            test_forget_ip,
            test_get_last_email,
            invite,
            invite_post,
            register_post,
            verify_email_ok,
            verify_email_template,
            verify_email,
            verify_email_nobody,
            verify_ip_ok,
            verify_ip_template,
            verify_ip,
            verify_ip_nobody,
            email_error,
            ip_error,
            status_auth_user,
            status_verified_user,
            status_user,
            status,
            ok_verified_user,
            ok_user,
            ok,
            logout,
            auth_user
        ],
    )
}
