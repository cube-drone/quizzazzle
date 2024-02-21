
use rocket::{Build, Rocket};
use rocket::State;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::uuid::Uuid;
use qrcode::QrCode;
use qrcode::render::svg;

use crate::Services;

#[derive(Responder)]
#[response(content_type = "image/svg+xml")]
struct QrCodeResponse(String);

#[get("/invite/<invite_code>")]
async fn qr(
    services: &State<Services>,
    invite_code: Uuid,
) -> Result<QrCodeResponse, Status> {

    // do some QR stuff here
    let config_public_address = services.config_get_public_address();
    let combined_link = format!("{}/auth/invite/{}", config_public_address, invite_code.to_string());

    let code = QrCode::new(combined_link).unwrap();

    let image = code.render::<svg::Color>()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("black"))
        .light_color(svg::Color("white"))
        .build();

    Ok(QrCodeResponse(image))
}

pub fn mount_routes(app: Rocket<Build>) -> Rocket<Build> {
    app.mount(
        "/qr",
        routes![
            qr,
        ],
    )
}
