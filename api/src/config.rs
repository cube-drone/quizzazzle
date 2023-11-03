struct Config {
    scyllaUrl: String,
}

fn config() -> _ {
    let scyllaUrl = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let conf = Config {
        scyllaUrl: scyllaUrl,
    };
}