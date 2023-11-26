use rocket::http::Status;

pub fn error_response(error_string: String) -> Status {
    if error_string.contains("Validation") || error_string.contains("400") {
        eprintln!("Validation Error: {}", error_string);
        return Status::BadRequest;
    }

    eprintln!("ERROR!: {}", error_string);
    return Status::InternalServerError;
}

#[macro_export]
macro_rules! no_shit {
    ($message:expr) => {
        $message.map_err(|err| crate::error::error_response(err.to_string()))?
    };
}
