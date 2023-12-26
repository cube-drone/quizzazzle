use rand::Rng;
use redis::AsyncCommands;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{Template, context};
use rocket::http::CookieJar;
use serde::Serialize;
use std::collections::HashMap;
use validator::Validate;

use crate::no_shit;
use crate::Services;

use crate::basic::types::{BasicThingCreate, BasicThingPublic};

#[get("/")]
fn root() -> &'static str {
    "Test functionality lives here!"
}

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello world"
}

#[get("/400")]
async fn bad_request() -> Status {
    Status::BadRequest
}

#[get("/418")]
async fn teapot() -> Status {
    Status::ImATeapot
}

#[get("/429")]
async fn cool_your_jets() -> Status {
    Status::TooManyRequests
}

#[get("/500")]
async fn internal_server_error() -> Status {
    Status::InternalServerError
}

#[get("/redirect")]
async fn redirect() -> Redirect {
    Redirect::to("/basic/hello".to_string())
}

#[get("/counter")]
async fn counter(services: &State<Services>) -> Result<String, Status> {
    let mut redis_connection = no_shit!(services.cache_redis.get_async_connection().await);
    let counter_result: i64 = no_shit!(redis_connection.incr("counter", 1).await);

    Ok(format!("Counter: {counter_result}"))
}
//
// Let's do some JSON, hoss
//

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ExampleSubObject {
    up: bool,
    down: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Pizza {
    crust: String,
    toppings: Option<Vec<String>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ExampleObject {
    username: String,
    timestamp_ms: u128,
    active: bool,
    sub_objects: Vec<ExampleSubObject>,
    maybe: Option<String>,
    maybe_not: Option<String>,
    pizzas: Option<HashMap<String, Pizza>>,
}

#[get("/json")]
async fn simple_json() -> Result<Json<ExampleObject>, Status> {
    let timestamp = no_shit!(std::time::UNIX_EPOCH.elapsed());
    let timestamp_ms = timestamp.as_millis();

    Ok(Json(ExampleObject {
        username: "harbo".to_string(),
        timestamp_ms: timestamp_ms,
        active: true,
        sub_objects: vec![
            ExampleSubObject {
                up: true,
                down: false,
            },
            ExampleSubObject {
                up: false,
                down: true,
            },
        ],
        maybe: Some("yes".to_string()),
        maybe_not: None,
        pizzas: Some(HashMap::from([
            (
                "peppy".to_string(),
                Pizza {
                    crust: "thicc".to_string(),
                    toppings: Some(vec!["cheese".to_string(), "pepperoni".to_string()]),
                },
            ),
            (
                "flatbread".to_string(),
                Pizza {
                    crust: "thinn".to_string(),
                    toppings: None,
                },
            ),
        ])),
    }))
}

#[get("/id/<id>")]
async fn simple_uuid(id: Uuid) -> String {
    // if you enter the Uuid correctly, you get this hello
    // if you screw it up, you get a 422: Unprocessable Entity
    format!("hello, {}", id)
}

#[get("/id")]
async fn id_redirect() -> Redirect {
    let uuid = Uuid::new_v4();
    // note that the uri! macro will calculate the endpoint for us, but it's not aware of the mount point
    Redirect::to(format!("/basic{}", uri![simple_uuid(uuid)]))
}

#[get("/coin")]
async fn coinflip() -> String {
    let mut rng = rand::thread_rng();
    let coin = rng.gen_bool(0.5);
    if coin {
        "heads".to_string()
    } else {
        "tails".to_string()
    }
}

// okay, next we're gonna try to do a complete database looparoo

#[post("/thing", format = "json", data = "<basic_thing_serialized>")]
async fn create_basic(
    services: &State<Services>,
    basic_thing_serialized: Json<BasicThingCreate>,
) -> Result<Json<BasicThingPublic>, Status> {
    // we have to deserialize the JSON into a struct
    let basic_thing_to_create = basic_thing_serialized.into_inner();
    no_shit!(basic_thing_to_create.validate());

    let basic_thing_public =
        no_shit!(crate::basic::view::create_basic_thing(&services, basic_thing_to_create).await);

    Ok(Json(basic_thing_public))
}

#[get("/thing/<id>")]
async fn get_basic(
    services: &State<Services>,
    id: Uuid,
) -> Result<Option<Json<BasicThingPublic>>, Status> {
    let basic_thing_public = no_shit!(crate::basic::view::get_basic_thing(&services, &id).await);

    match basic_thing_public {
        Some(basic_thing_public) => Ok(Some(Json(basic_thing_public))),
        None => Ok(None),
    }
}

#[get("/template")]
async fn get_template() -> Template {
    Template::render("basic", context! {
        foo: 123,
    })
}

#[get("/videotest")]
async fn get_videotest() -> Template {
    Template::render("videotest", context! {
        foo: 123,
    })
}

#[get("/setcookie")]
async fn setcookie(cookies: &CookieJar<'_>) -> String {

    cookies.add(("potato", "bark bark bark"));
    cookies.add_private(("secret", "pork pork pork"));

    "Cookie set".to_string()
}

#[get("/getcookie")]
async fn getcookie(cookies: &CookieJar<'_>) -> String {

    let maybe_potato = cookies.get("potato");
    let maybe_secret = cookies.get_private("secret");

    match maybe_potato{
        Some(cookieval) => println!("cookie {}", cookieval),
        _ => (),
    }
    match maybe_secret{
        Some(cookieval) => println!("secret {}", cookieval),
        _ => (),
    }

    "Cookie get".to_string()
}

use ::argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[get("/argon2")]
async fn argon2() -> Result<String, Status> {
    let password = Uuid::new_v4().to_string();
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let password_hash = no_shit!(argon2.hash_password(password.as_bytes(), &salt)).to_string();

    let parsed_hash = no_shit!(PasswordHash::new(&password_hash));
    assert!(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok());

    Ok(password_hash)
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/basic",
        routes![
            root,
            hello,
            bad_request,
            teapot,
            cool_your_jets,
            internal_server_error,
            redirect,
            counter,
            simple_json,
            simple_uuid,
            id_redirect,
            coinflip,
            create_basic,
            get_basic,
            get_template,
            get_videotest,
            setcookie,
            getcookie,
            argon2
        ],
    )
}
