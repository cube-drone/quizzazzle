
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use std::collections::HashMap;

use crate::no_shit;
use crate::Services;

#[get("/public")]
async fn public_config(services: &State<Services>) -> Result<Json<HashMap<String, String>>, Status> {
    let config_service = no_shit!(services.config.read());

    Ok(Json(config_service.public_config.clone()))
}

#[get("/private")]
async fn private_config(services: &State<Services>) -> Result<Json<HashMap<String, String>>, Status> {
    if services.is_production {
        return Err(Status::Forbidden);
    }

    let config_service = no_shit!(services.config.read());

    Ok(Json(config_service.private_config.clone()))
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/config",
        routes![
            public_config,
            private_config,
        ],
    )
}
