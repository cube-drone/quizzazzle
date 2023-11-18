use std::collections::HashMap;
use rand::Rng;
use rocket::{Rocket, Build, State};
use rocket::response::Redirect;
use rocket::http::Status;
use redis::AsyncCommands;
use rocket::serde::{Serialize, json::Json};
use rocket::serde::uuid::Uuid;

use crate::no_shit;

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
async fn counter(services: &State<crate::Services>) -> Result<String, Status> {
    let mut redis_connection = no_shit!( services.cache_redis.get_async_connection().await );
    let counter_result:i64 = no_shit!( redis_connection.incr("counter", 1).await );

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

	let timestamp = no_shit!( std::time::UNIX_EPOCH.elapsed() );
	let timestamp_ms = timestamp.as_millis();

	Ok(Json(ExampleObject{
		username: "harbo".to_string(),
		timestamp_ms: timestamp_ms,
		active: true,
		sub_objects: vec![ExampleSubObject{up: true, down: false}, ExampleSubObject{up: false, down:true}],
		maybe: Some("yes".to_string()),
		maybe_not: None,
		pizzas: Some(HashMap::from([
			("peppy".to_string(), Pizza{
				crust: "thicc".to_string(),
				toppings: Some(vec!["cheese".to_string(), "pepperoni".to_string()]),
			}),
			("flatbread".to_string(), Pizza{
				crust: "thinn".to_string(),
				toppings: None,
			})
		]))
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
	Redirect::to(format!("/basic{}",uri![simple_uuid(uuid)]))
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


pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount("/basic", routes![
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
		coinflip
	])
}