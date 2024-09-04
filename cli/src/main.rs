#[macro_use]
extern crate rocket;

use std::env;
use std::path::Path;
use url::Url;
use anyhow::{Result, anyhow};

use ministry_directory::MinistryDirectory;
use rocket::{Build, Rocket};
use rocket::response::content;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;
use indoc::indoc; // this is a macro that allows us to write multi-line strings in a more readable way
use serde::Serialize;

mod ministry_directory;
mod file_modifiers;

const APP_JS: &str = include_str!("../../js/build/feed.js");
const APP_CSS: &str = include_str!("../../js/build/style.css");
//const APP_FAVICON: &str = include_str!("../../target/logo.svg");

///
/// Initialize a new deck in the current directory
///
/// if the directory already contains a deck, this will fail unless --force is passed
///
fn init(flags: Flags){
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    directory.init(flags.force).expect("Failed to initialize directory.");
}

fn status(_flags: Flags){
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    directory.get_metadata().expect("Failed to get status.");
}

fn assets(_flags: Flags){
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    directory.compile_assets().expect("Failed to compile assets.");
}

#[get("/js/feed.js")]
async fn js_app() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(APP_JS)
}
#[get("/js/style.css")]
async fn js_css() -> content::RawCss<&'static str> {
    content::RawCss(APP_CSS)
}

#[derive(Clone)]
pub struct Flags{
    multi: bool,
    force: bool,
}

impl Flags{
    fn empty() -> Flags{
        Flags{
            multi: false,
            force: false,
        }
    }

    fn from_args(args: Vec<String>) -> Flags{
        let mut multi = false;
        let mut force = false;
        for arg in args{
            if arg == "multi" || arg == "--multi" || arg == "-m" || std::env::var("ROCKET_MULTI").unwrap_or("false".to_string()) == "true"{
                multi = true;
            }
            if arg == "force" || arg == "--force" || arg == "-f" || std::env::var("ROCKET_FORCE").unwrap_or("false".to_string()) == "true"{
                force = true;
            }
        }
        Flags{
            multi,
            force,
        }
    }
}

#[derive(Clone)]
pub struct Config{
    server_url: Url,
    site_name: String,
    default_locale: String,
    asset_directory: String,
    temporary_asset_directory: String,
    max_height: u32,
    max_width: u32,
    webp_quality: f32,
}

impl Config{
    fn from_env() -> Config{
        let server_url = std::env::var("ROCKET_SERVER_URL").unwrap_or("http://localhost:8000".to_string());
        let site_name = std::env::var("ROCKET_SITE_NAME").unwrap_or("Ministry".to_string());
        let default_locale = std::env::var("ROCKET_DEFAULT_LOCALE").unwrap_or("en_US".to_string());
        let asset_directory = std::env::var("ROCKET_ASSET_DIRECTORY").unwrap_or("./assets".to_string());
        let temporary_asset_directory = std::env::var("ROCKET_TEMPORARY_ASSET_DIRECTORY").unwrap_or("./temp_assets".to_string());
        Config{
            server_url: Url::parse(&server_url).unwrap(),
            site_name,
            default_locale,
            asset_directory,
            temporary_asset_directory,
            max_height: 800,
            max_width: 660,
            webp_quality: 30f32,
        }
    }
}

fn index_template(directory: MinistryDirectory, config: &State<Config>) -> Result<String> {
    if !directory.exists()?{
        return Err(anyhow!("Directory does not exist"));
    }
    let deck_metadata = directory.get_metadata()?;
    let title = deck_metadata.title;
    let author = deck_metadata.author;
    let description = match deck_metadata.description {
        Some(description) => description,
        None => "".to_string(),
    };
    let server_url = config.server_url.as_str();
    let url = format!("{}/{}/{}", server_url, deck_metadata.author_slug, deck_metadata.slug);
    let image = match deck_metadata.image_url {
        Some(image_url) => format!("{}/{}/{}/{}", server_url, deck_metadata.author_slug, deck_metadata.slug, image_url),
        None => "".to_string(),
    };
    let site_name = config.site_name.clone();
    let locale = match deck_metadata.locale {
        Some(locale) => locale,
        None => config.default_locale.clone(),
    };
    return Ok(format!(indoc!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <meta charset="UTF-8">
            <title>{}</title>
            <meta name="viewport" content="width=device-width">
            <meta property="og:title" content="{}" />
            <meta property="og:description" content="{}" />
            <meta property="article:author" content="{}" />
            <meta property="og:url" content="{}" />
            <meta property="og:site_name" content="{}" />
            <meta property="og:locale" content="{}" />
            <meta property="og:image" content="{}" />
            <link rel="stylesheet" href="/js/style.css">
            <link rel="icon" type="image/svg+xml" href="/favicon.svg" sizes="any"/>
        </head>
        <body>
            <div id="app"/>
            <script src="/js/feed.js"></script>
        </body>
    </html>
    "#), title, title, description, author, url, site_name, locale, image));
}

fn error_template(message: &str) -> String {
    return format!(indoc!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width">
            <title>Error</title>
        </head>
        <body>
            <h1>Error</h1>
            <p>{}</p>
        </body>
    </html>
    "#), message);
}

#[get("/")]
fn home(flags: &State<Flags>, config: &State<Config>) -> content::RawHtml<String> {
    if flags.multi{
        println!("loading multi-deck index");
    }
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    let rendered = index_template(directory, config);
    match rendered{
        Ok(html) => content::RawHtml(html),
        Err(e) => content::RawHtml(error_template(&e.to_string())),
    }
}

#[get("/s/<author_slug>/<deck_slug>")]
fn deck_home(flags: &State<Flags>, config: &State<Config>, author_slug: String, deck_slug: String) -> content::RawHtml<String> {
    if flags.multi{
        println!("loading multi-deck index");
    }
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    let rendered = index_template(directory, config);
    match rendered{
        Ok(html) => content::RawHtml(html),
        Err(e) => content::RawHtml(error_template(&e.to_string())),
    }
}

#[derive(Serialize)]
pub struct Index{
    id: String,
    metadata: ministry_directory::DeckMetadata,
    deck_ids: Vec<String>,
}

fn get_index(directory: MinistryDirectory) -> Result<Index> {
    let metadata = directory.get_metadata()?;
    let deck = directory.get_deck()?;
    Ok(Index{
        id: format!("{}/{}", metadata.author_slug, metadata.slug),
        metadata,
        deck_ids: deck.into_iter().map(|card| card.id).collect(),
    })
}

#[get("/s/<author_slug>/<deck_slug>/index")]
fn deck_index(author_slug: String, deck_slug: String, flags: &State<Flags>) -> Result<Json<Index>, Status> {
    let directory: MinistryDirectory;
    if flags.multi{
        println!("loading multi-deck index for {}/{}", author_slug, deck_slug);
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    else{
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    match get_index(directory){
        Ok(index) => Ok(Json(index)),
        Err(err) => {
            println!("Error getting index: {}", err);
            Err(Status::InternalServerError)
        },
    }
}
#[get("/index")]
fn default_index() -> Result<Json<Index>, Status> {
    let directory: MinistryDirectory;
    directory = ministry_directory::MinistryDirectory::new(".".to_string());
    match get_index(directory){
        Ok(index) => Ok(Json(index)),
        Err(err) => {
            println!("Error getting index: {}", err);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/s/<author_slug>/<deck_slug>/range/<start_id>/<end_id>")]
fn deck_range(author_slug: String, deck_slug: String, start_id: String, end_id: String, flags: &State<Flags>) -> Result<Json<Vec<ministry_directory::Card>>, Status> {
    let directory: MinistryDirectory;
    if flags.multi{
        println!("loading multi-deck index for {}/{}", author_slug, deck_slug);
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    else{
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    let deck = directory.get_deck();
    match deck {
        Ok(deck) => {
            // find the start and end indices
            let start = deck.iter().position(|card| card.id == start_id).unwrap_or(0);
            let mut end = deck.iter().position(|card| card.id == end_id).unwrap_or(deck.len());
            if end < start {
                return Err(Status::BadRequest);
            }
            if end < deck.len(){
                // we want to include the end card
                end += 1;
            }
            Ok(Json(deck[start..end].to_vec()))
        },
        Err(err) => {
            println!("Error getting deck: {}", err);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/s/<author_slug>/<deck_slug>/content/<content_id>")]
fn deck_id(author_slug: &str, deck_slug: &str, content_id: &str, flags: &State<Flags>) -> Result<Json<ministry_directory::Card>, Status> {
    let directory: MinistryDirectory;
    if flags.multi{
        println!("loading multi-deck index for {}/{}", author_slug, deck_slug);
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    else{
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    let deck = directory.get_deck();
    match deck {
        Ok(deck) => {
            //find the matching card
            let index = deck.iter().position(|card| card.id == content_id).unwrap_or(0);
            Ok(Json(deck[index].clone()))
        },
        Err(err) => {
            println!("Error getting deck: {}", err);
            Err(Status::InternalServerError)
        },
    }
}


#[get("/s/<author_slug>/<deck_slug>/assets/<asset_path..>?<file_directives..>")]
async fn deck_assets(author_slug: &str, deck_slug: &str, asset_path: std::path::PathBuf, file_directives: file_modifiers::FileDirectives, flags: &State<Flags>, config: &State<Config>) -> Result<rocket::fs::NamedFile, Status> {
    let directory: MinistryDirectory;
    if flags.multi{
        println!("loading multi-deck index for {}/{}", author_slug, deck_slug);
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    else{
        directory = ministry_directory::MinistryDirectory::new(".".to_string());
    }
    match directory.get_named_file(asset_path, &config, &file_directives).await{
        Ok(opened_file) => Ok(opened_file),
        Err(err) => {
            println!("Error getting asset: {}", err);
            Err(Status::NotFound)
        },
    }
}

#[get("/assets/<asset_path..>?<file_directives..>")]
async fn default_assets(asset_path: std::path::PathBuf, file_directives: file_modifiers::FileDirectives, config: &State<Config>) -> Result<rocket::fs::NamedFile, Status> {
    let directory: MinistryDirectory;
    directory = ministry_directory::MinistryDirectory::new(".".to_string());
    match directory.get_named_file(asset_path, &config, &file_directives).await{
        Ok(opened_file) => Ok(opened_file),
        Err(err) => {
            println!("Error getting asset: {}", err);
            Err(Status::NotFound)
        },
    }
}

async fn launch_server(flags: Flags, config: Config) -> Rocket<Build> {

    let mut app = rocket::build();

    app = app.mount("/", routes![
        home,
        deck_home,
        deck_index,
        default_index,
        deck_range,
        deck_id,
        deck_assets,
        default_assets,
    ]);

    if std::env::var("ROCKET_ENV").unwrap_or("development".to_string()) == "development"{
        // here we point to the JS and CSS build directories:
        // we only bother with this next bit if we're in dev mode: otherwise we should use include_str! to bundle the files directly into the binary
        let dev_ui_location = std::env::var("JS_BUILD_LOCATION").unwrap_or("../js/build".to_string());
        //if location exists:
        match Path::new(&dev_ui_location).exists(){
            true => {
                println!("Serving from: {}", dev_ui_location);
                app = app.mount("/js", FileServer::from(dev_ui_location));
            },
            false => {
                println!("No JS build location found at: {}", dev_ui_location);
                app = app.mount("/", routes![js_app, js_css]);
            }
        }
    }
    else{
        app = app.mount("/", routes![js_app, js_css]);
    }

    app = app.manage(flags);
    app = app.manage(config);

    // if _multi is false, we are running in single-deck mode
    // the only goal here is to serve the deck in the current directory
    // as well as the JS required to run it
    // we're assuming that we're running in "dev" mode, which is to say, frequently updating the deck
    // (I guess we could also have a "prod" mode where everything is aggressively cached, but
    //     the assumption is that "multi" is the REAL prod mode)

    /*
    app = app.manage(services);

    app = app.register("/", catchers![
        error::not_found,
        error::you_done_fucked_up,
        error::unprocessable,
        error::server_error
    ]);

	// Mount Routes
    app = app.mount("/static", FileServer::from("../js/static"));
    app = app.mount("/build", FileServer::from("../js/build"));
    */

    app
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // Parse any args that were passed in:
    let args: Vec<String> = env::args().collect();

    let flags = Flags::empty();
    let config = Config::from_env();

    if args.len() == 1{
        println!("Help:");
        println!("  init:       Create a new deck in the current directory");
        println!("  status:     Show the status of the deck");
        println!("  assets:     Compile all assets into a built assets directory");
        println!("  diff:       Diff the current deck against the last published deck");
        println!("  serve:      Start the server in single-deck mode, using the deck in the current directory");
        println!("  multi:      Start the server in multi-deck mode");
        println!("  login:      Log-in to a multi-deck server.");
        println!("  publish:    Publish (upload) the current deck to a multi-deck server.");
        std::process::exit(0);
    }
    if args.len() > 1{
        let flags = Flags::from_args(args.clone());

        let arg = &args[1];
        if arg == "init"{
            init(flags);
            std::process::exit(0);
        }
        if arg == "status"{
            println!("Status...");
            status(flags);
            std::process::exit(0);
        }
        if arg == "assets"{
            println!("Assets...");
            assets(flags);
            std::process::exit(0);
        }
        if arg == "diff"{
            println!("Diffing...");
            std::process::exit(0);
        }
        if arg == "login"{
            println!("Logging in...");
            std::process::exit(0);
        }
        if arg == "publish"{
            println!("Publishing...");
            std::process::exit(0);
        }
        if arg == "serve"{
            println!("Serving...");
        }
        if arg == "multi"{
            // a multi server is the final production server this project is intended to be run on
            // it is a server that can host multiple decks, multiple users, etc.
            println!("Multi...");
        }
    }

    launch_server(flags, config).await
}