use rocket::http::Status;

pub fn error_response(error_string: String) -> Status {
    eprintln!("ERROR!: {}", error_string);
    return Status::InternalServerError;
}

#[macro_export]
macro_rules! no_shit {
    ($message:expr) => {
        $message.map_err(|err| crate::error::error_response(err.to_string()))?
    }
}