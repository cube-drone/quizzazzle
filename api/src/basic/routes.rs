use rocket::{Rocket, Build, State};
use rocket::http::Status;
use redis::AsyncCommands;

use crate::no_shit;

#[get("/")]
fn root() -> &'static str {
	"Test functionality lives here!"
}

#[get("/counter")]
async fn counter(services: &State<crate::Services>) -> Result<String, Status> {
    let mut redis_connection = no_shit!( services.cache_redis.get_async_connection().await );
    let counter_result:i64 = no_shit!( redis_connection.incr("counter", 1).await );

    Ok(format!("Counter: {counter_result}"))
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

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount("/basic", routes![root, counter, bad_request, teapot, cool_your_jets, internal_server_error])
}