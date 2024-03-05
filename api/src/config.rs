use std::env;

pub fn public_address() -> String {
    env::var("PUBLIC_ADDRESS").unwrap_or("http://localhost:3333".to_string())
}