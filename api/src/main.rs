#[macro_use]
extern crate rocket;

use std::fs;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;
//use rocket::http::hyper::service;
use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket::tokio;
use redis::Client;
use redis::AsyncCommands;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::session::Session;
use scylla::SessionBuilder;
use comrak::{markdown_to_html, Options};

use moka::future::Cache;

use tera::{Value, to_value};

use crate::auth::model::UserId;

mod config;
mod fairings;
mod error; // provides the no_shit! macro
mod icons;

mod email;
mod home;
mod auth;
mod feed;
mod qr;
mod services;

use crate::services::background_tick::RequiresBackgroundTick;

/*
    Services gets passed around willy nilly between threads so it needs to be cram-packed fulla arcs like a season of Naruto
*/
#[derive(Clone)]
pub struct ScyllaService {
    pub session: Arc<Session>,
    pub prepared_queries: Arc<HashMap<&'static str, PreparedStatement>>,
}

/*
    Note that this is private and public in the "visible to the end-user" sense, not in the "OO" sense
*/
#[derive(Clone)]
pub struct Services {
    pub is_production: bool,
    pub email_token_service: Arc<services::disposable_token_service::DisposableTokenService<UserId>>,
    pub ip_token_service: Arc<services::disposable_token_service::DisposableTokenService<UserId>>,
    pub password_token_service: Arc<services::disposable_token_service::DisposableTokenService<UserId>>,
    pub auth_token_service: Arc<services::auth_token_service::AuthTokenService<crate::auth::model::UserSession>>,
    pub rate_limit_service: Arc<services::rate_limit::RateLimitService>,
    pub cache_redis: Arc<Client>,
    pub application_redis: Arc<Client>,
    pub scylla: ScyllaService,
    pub email: Arc<email::EmailProvider>,
    pub static_markdown: Arc<HashMap<&'static str, String>>,
    pub local_cache: Arc<Cache<String, String>>,
}

async fn setup_redis(redis_url: &String) -> Arc<Client> {
    // Redis Setup
    let client = Client::open(redis_url.clone()).expect("Could not create Redis client");
    let mut connection = client
        .get_async_connection()
        .await
        .expect("Could not connect to Redis");

    // Redis Test
    let _: () = connection
        .set("foo", "bar")
        .await
        .expect("Could not test Redis connection");
    let result: String = connection
        .get("foo")
        .await
        .expect("Could not test Redis connection");
    assert_eq!(result, "bar");

    Arc::new(client)
}

async fn setup_scylla_cluster(scylla_url: &String) -> Arc<Session> {
    let session: Session = SessionBuilder::new()
        .known_node(scylla_url)
        .build()
        .await
        .expect("Could not connect to Scylla");

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 2}", &[]).await.expect("Could not create keyspace");

    Arc::new(session)
}

//  "tos.md" -> look for /tmp/api-static/tos.md -> read it -> markdownify it -> return it
fn static_markdownify(file_name: &str) -> String {
    let file_path = format!("/tmp/api-static/{}", file_name);
    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let html = markdown_to_html(&file_contents, &Options::default());

    html
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // Environment Variables
    let is_production: bool = env::var("GROOVELET_PRODUCTION").unwrap_or_else(|_| "false".to_string()) == "true";
    let cache_redis_url = env::var("CACHE_REDIS_URL").unwrap_or_else(|_| "".to_string());
    let application_redis_url =
        env::var("APPLICATION_REDIS_URL").unwrap_or_else(|_| "".to_string());

    // Scylla Setup
    let scylla_url = env::var("SCYLLA_URL").unwrap_or_else(|_| "".to_string());
    let scylla_connection = setup_scylla_cluster(&scylla_url).await;
    let mut prepared_queries: HashMap<&'static str, PreparedStatement> = HashMap::new();

    // TECHNICALLY this is MibiBytes, not MegaBytes, but in my defense: I don't care
    let cache_megabytes_string = env::var("GROOVELET_CACHE_MEGABYTES").unwrap_or_else(|_| "512".to_string());
    let cache_megabytes = cache_megabytes_string.parse::<u64>().expect("Could not parse cache size properly into u64");
    let cache_bytes = cache_megabytes * 1024 * 1024;

    // This cache will hold up to cache_megabytes MiB of values.
    let cache: Cache<String, String> = Cache::builder()
        // A weigher closure takes &K and &V and returns a u32 representing the
        // relative size of the entry. Here, we use the byte length of the value
        // String as the size.
        .weigher(|_key, value: &String| -> u32 {
            value.len().try_into().unwrap_or(u32::MAX)
        })
        .max_capacity(cache_bytes)
        .build();

    cache.insert("hello".to_string(), "world".to_string()).await;

    let hi = cache.get("hello").await.expect("Moka cache is broken");
    assert_eq!(hi, "world");

    let data_directory = "./data".to_string();
    let three_days_in_seconds = 60 * 60 * 24 * 3;
    let drop_table_on_start = !is_production;

    let email_verification_token_service_options = services::disposable_token_service::DisposableTokenServiceOptions{
        data_directory: data_directory.clone(),
        name: "email_verification".to_string(),
        cache_capacity: 10000,
        expiry_seconds: three_days_in_seconds,
        drop_table_on_start: drop_table_on_start,
        get_refreshes_expiry: false,
        probability_of_refresh: 0.0,
    };
    let email_verification_token_service = services::disposable_token_service::DisposableTokenService::<UserId>::new(email_verification_token_service_options)
        .expect("Could not create email verification token service");

    let ip_verification_token_service_options = services::disposable_token_service::DisposableTokenServiceOptions{
        data_directory: data_directory.clone(),
        name: "ip_verification".to_string(),
        cache_capacity: 10000,
        expiry_seconds: three_days_in_seconds,
        drop_table_on_start: drop_table_on_start,
        get_refreshes_expiry: false,
        probability_of_refresh: 0.0,
    };
    let ip_verification_token_service = services::disposable_token_service::DisposableTokenService::<UserId>::new(ip_verification_token_service_options)
        .expect("Could not create ip verification token service");

    let password_change_token_service_options = services::disposable_token_service::DisposableTokenServiceOptions{
        data_directory: data_directory.clone(),
        name: "password_change".to_string(),
        cache_capacity: 10000,
        expiry_seconds: three_days_in_seconds,
        drop_table_on_start: drop_table_on_start,
        get_refreshes_expiry: false,
        probability_of_refresh: 0.0,
    };
    let password_change_token_service = services::disposable_token_service::DisposableTokenService::<UserId>::new(password_change_token_service_options)
        .expect("Could not create password change token service");


    let seven_days_in_seconds = 60 * 60 * 24 * 7;
    let auth_token_service_options = services::auth_token_service::AuthTokenServiceOptions{
        data_directory: data_directory.clone(),
        name: "auth".to_string(),
        cache_capacity: 100000,
        expiry_seconds: seven_days_in_seconds,
        drop_table_on_start: drop_table_on_start,
        max_tokens_per_user: 8,
    };

    let auth_token_service = services::auth_token_service::AuthTokenService::<crate::auth::model::UserSession>::new(auth_token_service_options)
        .expect("Could not create auth token service");

    let rate_limit_service = services::rate_limit::RateLimitService::new();

    // Initialize Models & Prepare all Scylla Queries
    let mut auth_prepared_queries = auth::model::initialize(&scylla_connection)
        .await
        .expect("Could not initialize auth model");
    let mut other_prepared_queries: HashMap<&'static str, PreparedStatement> = HashMap::new();

    let queries_to_merge = vec![
        &mut auth_prepared_queries,
        &mut other_prepared_queries
    ];

    for query_map in queries_to_merge {
        prepared_queries.extend(query_map.drain());
    }

    // Static Content Setup
    let mut static_hashmap = HashMap::new();
    static_hashmap.insert("tos", static_markdownify("tos.md"));
    static_hashmap.insert("faq", static_markdownify("faq.md"));
    static_hashmap.insert("pricing", static_markdownify("pricing.md"));

    // Email Setup
    let email_provider = email::EmailProvider::setup(is_production).await;
    let test_email = email::EmailAddress::new("test@gooble.email".to_string()).expect("Could not create test email");
    email_provider.send_hello(&test_email).await.expect("Could not send email");

    // Service Setup
    let services = Services {
        is_production: is_production,
        email_token_service: Arc::new(email_verification_token_service),
        ip_token_service: Arc::new(ip_verification_token_service),
        password_token_service: Arc::new(password_change_token_service),
        auth_token_service: Arc::new(auth_token_service),
        rate_limit_service: Arc::new(rate_limit_service),
        cache_redis: setup_redis(&cache_redis_url).await,
        application_redis: setup_redis(&application_redis_url).await,
        scylla: ScyllaService {
            session: scylla_connection,
            prepared_queries: Arc::new(prepared_queries),
        },
        email: Arc::new(email_provider),
        static_markdown: Arc::new(static_hashmap),
        local_cache: Arc::new(cache),
    };

    let services_clone = services.clone();

    // Create a root user
    services.create_root_user().await.expect("Could not create root user");

	// Launch App
    let mut app = rocket::build();

    app = app.manage(services);
    app = app.attach(crate::fairings::timing::RequestTimer)
             .attach(crate::fairings::rate_limit::RateLimit{
                 ip_limit: Arc::new(Cache::builder().max_capacity(100000).time_to_idle(Duration::from_millis(5000)).build())
             })
             .attach(crate::fairings::poweredby::PoweredBy)
             .attach(Template::custom(move |engines|{
                // register a few important variables like the site name

                engines.tera.register_function("public_address", move |_args: &HashMap<String, Value>| {
                    let value = crate::config::public_address();
                    Ok(to_value(value)?)
                });

                // register the icons
                engines.tera.register_function("sbubby", icons::sbubby);
                engines.tera.register_function("icon_home", icons::icon_home);
                engines.tera.register_function("icon_profile", icons::icon_profile);
                engines.tera.register_function("icon_applications", icons::icon_applications);
                engines.tera.register_function("icon_relationships", icons::icon_relationships);
                engines.tera.register_function("icon_search", icons::icon_search);
                engines.tera.register_function("icon_circle_cross", icons::icon_circle_cross);
                engines.tera.register_function("icon_circle_check", icons::icon_circle_check);
                engines.tera.register_function("icon_circle_plus", icons::icon_circle_plus);
                engines.tera.register_function("icon_circle_minus", icons::icon_circle_minus);
                engines.tera.register_function("icon_circle_chevron_left", icons::icon_circle_chevron_left);
                engines.tera.register_function("icon_circle_chevron_up", icons::icon_circle_chevron_up);
                engines.tera.register_function("icon_circle_chevron_right", icons::icon_circle_chevron_right);
                engines.tera.register_function("icon_circle_chevron_down", icons::icon_circle_chevron_down);
                engines.tera.register_function("icon_circle_hamburger", icons::icon_circle_hamburger);
                engines.tera.register_function("icon_circle_question", icons::icon_circle_question);
                engines.tera.register_function("icon_mailbox", icons::icon_mailbox);
                engines.tera.register_function("icon_nervous", icons::icon_nervous);
                engines.tera.register_function("icon_exclamation", icons::icon_exclamation);
                engines.tera.register_function("icon_invitation", icons::icon_invitation);
             }));

    app = app.register("/", catchers![
        error::not_found,
        error::you_done_fucked_up,
        error::unprocessable,
        error::server_error
    ]);

	// Mount Routes
    app = app.mount("/static", FileServer::from("/tmp/static"));
    app = app.mount("/build", FileServer::from("/tmp/build"));

    // home is where "/" lives.
    app = home::routes::mount_routes(app);
    // auth: login, registration, that sort of stuff
    app = auth::routes::mount_routes(app);

    // qr: that qr code thing
    app = qr::mount_routes(app);

    // feed: we haven't worked out how feed works quite yet
    app = feed::routes::mount_routes(app);

    tokio::spawn(async move {
        loop{
            // code goes here
            println!("Every 30 seconds... ");

            let resp = &services_clone.email_token_service.background_tick();
            match resp{
                Ok(_) => {},
                Err(e) => {
                    println!("Background Error: Could not delete expired email tokens: {:?}", e);
                }
            }

            let resp = &services_clone.ip_token_service.background_tick();
            match resp{
                Ok(_) => {},
                Err(e) => {
                    println!("Background Error: Could not delete expired ip tokens: {:?}", e);
                }
            }

            let resp = &services_clone.password_token_service.background_tick();
            match resp{
                Ok(_) => {},
                Err(e) => {
                    println!("Background Error: Could not delete expired password tokens: {:?}", e);
                }
            }

            let resp = &services_clone.auth_token_service.background_tick();
            match resp{
                Ok(_) => {},
                Err(e) => {
                    println!("Background Error: Could not delete expired auth tokens: {:?}", e);
                }
            }

            // and now, I sleep
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });

    app
}
