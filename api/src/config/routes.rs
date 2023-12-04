
//use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use std::collections::HashMap;

//use crate::no_shit;
use crate::Services;

#[get("/public")]
async fn public_config(services: &State<Services>) -> Json<HashMap<String, String>> {
    let config_service = services.config.read().expect("oh shit, can't get config lock");
    //let json = serde_json::to_string(&configService.public_config).unwrap();

    Json(config_service.public_config.clone())
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/config",
        routes![
            public_config
        ],
    )
}
