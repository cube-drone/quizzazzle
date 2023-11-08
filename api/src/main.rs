#[macro_use] extern crate rocket;
mod config;
mod scylla_up;
use std::sync::Arc;
use rocket::State;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hooba")]
fn hooba(services: &State<Services>) -> &'static str {
    "Hooba, world!"
}

pub struct Services{
    pub scylla: Arc<scylla_up::Session>,
}

#[launch]
async fn rocket() -> _ {
    let conf: config::Config = config::config();
    let scylla = Arc::new(scylla_up::scylla_setup(&conf).await.unwrap());
    let services = Services{
        scylla: scylla,
    };

    rocket::build().manage(services)
        .mount("/", routes![index, hooba])
}
