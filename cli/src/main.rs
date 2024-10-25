#[macro_use]
extern crate rocket;

use std::env;
use std::path::Path;
use std::collections::HashMap;
use url::Url;
use anyhow::Result;

use ministry_directory::MinistryDirectory;
use rocket::{Build, Rocket};
use rocket::response::content;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;
use std::net::IpAddr;
use indoc::indoc; // this is a macro that allows us to write multi-line strings in a more readable way
use serde::Serialize;
use slugify::slugify;
use qrcode::QrCode;
use qrcode::render::svg;

use ministry_directory::{DeckMetadata, DeckSummary};
use ministry_directory_cache::MinistryDirectoryCache;

mod ministry_directory;
mod ministry_directory_cache;
mod file_modifiers;

const APP_JS: &str = include_str!("js/feed.js");
const APP_CSS: &str = include_str!("js/style.css");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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

fn new(flags: Flags){
    println!("What's your name?");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).expect("Failed to read line.");
    author = author.trim().to_string();
    println!("What's the name of your deck?");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("Failed to read line.");
    title = title.trim().to_string();
    let author_slug = slugify!(&author);
    let deck_slug = slugify!(&title);

    // create the directory ${author_slug}/$}{deck_slug}
    let directory_root = std::path::PathBuf::from(author_slug.clone()).join(deck_slug.clone());
    std::fs::create_dir_all(directory_root.clone()).expect("Failed to create directory.");

    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_str().unwrap_or_else(|| ".").to_string());
    directory.init_with_name(flags.force, title, author).expect("Failed to initialize directory.");
}

fn status(_flags: Flags){
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    let metadata = directory.get_metadata().expect("Failed to get status.");
    println!("Deck: {}", metadata.title);
}

#[get("/js/<_version>/feed.js")]
async fn js_app(_version: String) -> content::RawJavaScript<&'static str> {
    // we include the version, even though it's not used, so that when the browser caches the file
    //  a new version will be requested when the version changes
    content::RawJavaScript(APP_JS)
}
#[get("/js/<_version>/style.css")]
async fn js_css(_version: String) -> content::RawCss<&'static str> {
    // we include the version, even though it's not used, so that when the browser caches the file
    //  a new version will be requested when the version changes
    content::RawCss(APP_CSS)
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    indoc!(r#"
    User-agent: Bingbot
    Disallow:
    User-agent: DuckDuckBot
    Disallow:
    User-agent: BingPreview
    Disallow:
    User-agent: MicrosoftPreview
    Disallow:
    User-agent: Googlebot
    Disallow:
    User-agent: Googlebot Smartphone
    Disallow:
    User-agent: Googlebot Desktop
    Disallow:
    User-agent: Google-InspectionTool
    Disallow:
    User-agent: *
    Disallow: /
    "#)
}

#[derive(Clone)]
pub struct Flags{
    force: bool,
}

impl Flags{
    fn empty() -> Flags{
        Flags{
            force: false,
        }
    }

    fn from_args(args: Vec<String>) -> Flags{
        let mut force = false;
        for arg in args{
            if arg == "force" || arg == "--force" || arg == "-f" || std::env::var("ROCKET_FORCE").unwrap_or("false".to_string()) == "true"{
                force = true;
            }
        }
        Flags{
            force,
        }
    }
}

#[derive(Clone)]
pub struct Config{
    dev: bool,
    server_url: Url,
    site_name: String,
    default_locale: String,
    temporary_asset_directory: String,
    max_height: u32,
    max_width: u32,
    webp_quality: f32,
}

impl Config{
    fn from_env() -> Config{
        let dev = std::env::var("ROCKET_ENV").unwrap_or("production".to_string()) == "development";
        let server_url = std::env::var("ROCKET_SERVER_URL").unwrap_or("http://localhost:8000".to_string());
        let site_name = std::env::var("ROCKET_SITE_NAME").unwrap_or("CardChapter".to_string());
        let default_locale = std::env::var("ROCKET_DEFAULT_LOCALE").unwrap_or("en_US".to_string());
        let temporary_asset_directory = std::env::var("ROCKET_TEMPORARY_ASSET_DIRECTORY").unwrap_or("./temp_assets".to_string());
        Config{
            dev,
            server_url: Url::parse(&server_url).unwrap(),
            site_name,
            default_locale,
            temporary_asset_directory,
            max_height: 800,
            max_width: 660,
            webp_quality: 30f32,
        }
    }
}

pub struct Services{
    pub cache: MinistryDirectoryCache,
}

impl Services{
    pub fn new() -> Services{
        Services{
            cache: MinistryDirectoryCache::new(),
        }
    }
}

fn index_template(deck_metadata: DeckMetadata, config: &State<Config>, is_home: bool) -> Result<String> {
    let title = deck_metadata.title;
    let author = deck_metadata.author;
    let description = match deck_metadata.description {
        Some(description) => description,
        None => "".to_string(),
    };
    let favicon = match deck_metadata.favicon {
        Some(favicon_url) => favicon_url,
        None => "/assets/favicon.png".to_string(),
    };
    let server_url = config.server_url.as_str();
    let url = match is_home{
        false => format!("{}s/{}/{}", server_url, deck_metadata.author_slug, deck_metadata.slug),
        true => server_url.to_string(),
    };
    let image = match (is_home, deck_metadata.image_url) {
        (false, Some(image_url)) => format!("{}s/{}/{}/{}", server_url, deck_metadata.author_slug, deck_metadata.slug, image_url),
        (true, Some(image_url)) => format!("{}{}", server_url, image_url),
        _ => "".to_string(),
    };
    let site_name = config.site_name.clone();
    let locale = match deck_metadata.locale {
        Some(locale) => locale,
        None => config.default_locale.clone(),
    };
    let extra_header = deck_metadata.extra_header.clone().unwrap_or("".to_string());

    let js_location = match config.dev {
        true => "/js/feed.js",
        false => &format!("/js/{}/feed.js", VERSION),
    };

    let css_location = match config.dev {
        true => "/js/style.css",
        false => &format!("/js/{}/style.css", VERSION),
    };

    if deck_metadata.hidden {
        return Ok(error_template("This deck is hidden."));
    }

    return Ok(format!(indoc!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <!-- CardChapter Ministry Version: {} -->
            <link rel="stylesheet" href="{}">
            <link rel="icon" type="image/png" href="{}" sizes="any"/>
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
            {}
        </head>
        <body>
            <div id="app">

                <div class="primary-card">
                    <div class="content">
                        <header id="primary-header">
                        </header>
                        <div class="everything-feed">
                            <div class="frames">
                                <div class="loader-wrapper">
                                    <div class="loader">
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <script src="{}"></script>
        </body>
    </html>
    "#), VERSION, css_location, favicon, title, title, description, author, url, site_name, locale, image, extra_header, js_location));
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
async fn home(config: &State<Config>, services: &State<Services>) -> content::RawHtml<String> {
    let directory_root = ".";
    let metadata = services.cache.get_metadata(directory_root).await;

    match metadata{
        Ok(deck_metadata) => {
            let rendered = index_template(deck_metadata, config, true);

            match rendered{
                Ok(html) => content::RawHtml(html),
                Err(e) => content::RawHtml(error_template(&e.to_string())),
            }
        },
        Err(e) => content::RawHtml(error_template(&e.to_string())),
    }
}

#[get("/s/<author_slug>/<deck_slug>")]
async fn deck_home(config: &State<Config>, services: &State<Services>, author_slug: &str, deck_slug: &str) -> content::RawHtml<String> {
    let path = std::path::PathBuf::from(author_slug).join(deck_slug);
    let metadata = services.cache.get_metadata(path.to_str().unwrap_or_else(|| ".")).await;
    match metadata{
        Ok(deck_metadata) => {
            let rendered = index_template(deck_metadata, config, false);

            match rendered{
                Ok(html) => content::RawHtml(html),
                Err(e) => content::RawHtml(error_template(&e.to_string())),
            }
        },
        Err(e) => content::RawHtml(error_template(&e.to_string())),
    }
}

#[derive(Serialize)]
pub struct Index{
    id: String,
    metadata: DeckMetadata,
    deck_ids: Vec<String>,
    toc: Vec<ministry_directory::TableOfContentsEntry>,
    version: String,
}

async fn get_index(services: &State<Services>, directory_path: &str) -> Result<Index> {
    let metadata = services.cache.get_metadata(directory_path).await?;
    let deck = services.cache.get_deck(directory_path).await?;
    if metadata.hidden {
        return Err(anyhow::anyhow!("This deck is hidden."));
    }
    Ok(Index{
        id: format!("{}/{}", metadata.author_slug, metadata.slug),
        metadata,
        deck_ids: deck.clone().into_iter().map(|card| card.id).collect(),
        toc: deck.into_iter().map(|card| card.to_toc_entry()).collect(),
        version: VERSION.to_string(),
    })
}

#[get("/s/<author_slug>/<deck_slug>/index")]
async fn deck_index(services: &State<Services>, author_slug: &str, deck_slug: &str) -> Result<Json<Index>, Status> {
    let path = std::path::PathBuf::from(author_slug).join(deck_slug);
    match get_index(services, path.to_str().unwrap_or_else(|| ".")).await{
        Ok(index) => Ok(Json(index)),
        Err(err) => {
            println!("Error getting index: {}", err);
            Err(Status::InternalServerError)
        },
    }
}
#[get("/index")]
async fn default_index(services: &State<Services>) -> Result<Json<Index>, Status> {
    match get_index(services, ".").await{
        Ok(index) => Ok(Json(index)),
        Err(err) => {
            println!("Error getting index: {}", err);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/s/<author_slug>/<deck_slug>/range/<start_id>/<end_id>")]
async fn deck_range(services: &State<Services>, author_slug: &str, deck_slug: &str, start_id: &str, end_id: &str) -> Result<Json<Vec<ministry_directory::Card>>, Status> {
    let path = std::path::PathBuf::from(author_slug).join(deck_slug);
    let directory_path;
    if author_slug == "default" && deck_slug == "default"{
        directory_path = ".".to_string();
    }
    else{
        directory_path = path.to_str().unwrap_or_else(|| ".").to_string();
    }
    let metadata = services.cache.get_metadata(&directory_path).await;
    let deck = services.cache.get_deck(&directory_path).await;
    match (metadata, deck) {
        (Ok(metadata), Ok(deck)) => {
            if metadata.hidden {
                return Err(Status::NotFound);
            }
            // find the start and end indices
            let start: usize;
            if start_id == "0" || start_id == "undefined" || start_id == "" || start_id == "null" {
                start = 0;
            }
            else{
                start = deck.iter().position(|card| card.id == start_id).unwrap_or(0);
            }
            let mut end: usize;
            if end_id == "0" || end_id == "undefined" || end_id == "" || end_id == "null" {
                end = std::cmp::min(start + 100, deck.len());
            }
            else{
                end = deck.iter().position(|card| card.id == end_id).unwrap_or(deck.len());
            }
            if end < start {
                return Err(Status::BadRequest);
            }
            if end < deck.len(){
                // we want to include the end card
                end += 1;
            }
            Ok(Json(deck[start..end].to_vec()))
        },
        (Err(err), _) => {
            println!("Error getting deck metadata: {}", err);
            Err(Status::InternalServerError)
        },
        (_, Err(err)) => {
            println!("Error getting deck: {}", err);
            Err(Status::InternalServerError)
        },
    }
}

#[get("/s/<author_slug>/<deck_slug>/content/<content_id>")]
async fn deck_id(services: &State<Services>, author_slug: &str, deck_slug: &str, content_id: &str) -> Result<Json<ministry_directory::Card>, Status> {
    let path = std::path::PathBuf::from(author_slug).join(deck_slug);
    let metadata = services.cache.get_metadata(path.to_str().unwrap_or_else(|| ".")).await;
    let deck = services.cache.get_deck(path.to_str().unwrap_or_else(|| ".")).await;
    match (metadata, deck) {
        (Ok(metadata), Ok(deck)) => {
            if metadata.hidden {
                return Err(Status::NotFound);
            }
            //find the matching card
            let index = deck.iter().position(|card| card.id == content_id).unwrap_or(0);
            Ok(Json(deck[index].clone()))
        },
        (Err(err), _) => {
            println!("Error getting deck metadata: {}", err);
            Err(Status::InternalServerError)
        },
        (_, Err(err)) => {
            println!("Error getting deck: {}", err);
            Err(Status::InternalServerError)
        },
    }
}


#[get("/s/<author_slug>/<deck_slug>/assets/<asset_path..>?<file_directives..>")]
async fn deck_assets(author_slug: &str, deck_slug: &str, asset_path: std::path::PathBuf, file_directives: file_modifiers::FileDirectives, config: &State<Config>) -> Result<rocket::fs::NamedFile, Status> {
    let path = std::path::PathBuf::from(author_slug).join(deck_slug);
    let directory = ministry_directory::MinistryDirectory::new(path.to_str().unwrap_or_else(|| ".").to_string());

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

#[get("/sitemap")]
async fn sitemap(services: &State<Services>) -> Json<HashMap<String, Vec<DeckSummary>>> {
    // look at the current directory
    let path = std::path::PathBuf::from(".");
    let mut hash_map = HashMap::new();

    // for each subdirectory...
    let paths = std::fs::read_dir(path).unwrap();
    for author_path in paths{
        let author_path = author_path.unwrap().path();
        let str_path = author_path.to_str().unwrap_or("");
        if str_path == "." || str_path.ends_with(".") || str_path.ends_with(".git") || str_path.ends_with("temp_assets") ||
                str_path.ends_with("node_modules") || str_path.ends_with("assets") || str_path.ends_with("src") || str_path.ends_with("target") {
            continue;
        }
        if author_path.is_dir(){
            // if it's a directory, that's a _user_ directory: so "author_slug" is the name of the directory
            // for each subdirectory of that directory...
            // println!("Author path: {}", author_path.to_str().unwrap_or(""));
            let more_paths = std::fs::read_dir(author_path.clone()).unwrap();
            for deck_path in more_paths{
                let deck_path = deck_path.unwrap().path();
                let str_deck_path = deck_path.to_str().unwrap_or("");
                if str_deck_path == "." || str_deck_path.ends_with(".") || str_deck_path.ends_with(".git") || str_deck_path.ends_with("temp_assets") ||
                        str_deck_path.ends_with("node_modules") || str_deck_path.ends_with("assets") {
                    continue;
                }
                if deck_path.is_dir(){
                    // if it's a directory, that's a _deck_ directory: so "deck_slug" is the name of the directory
                    // for each file in that directory...
                    let deck = ministry_directory::MinistryDirectory::new(deck_path.to_str().unwrap_or("").to_string());
                    if deck.exists(){
                        let metadata = services.cache.get_metadata(deck_path.to_str().unwrap_or("")).await.unwrap();
                        if metadata.hidden || metadata.unlisted {
                            // don't include hidden or unlisted decks in the sitemap
                            continue;
                        }

                        let author_slug = author_path.to_str().unwrap_or("").to_string().replace(".\\", "");

                        if hash_map.contains_key(&author_slug){
                            let decks: &mut Vec<DeckSummary> = hash_map.get_mut(&author_slug).unwrap();
                            decks.push(metadata.to_summary());
                        }
                        else{
                            hash_map.insert(author_slug, vec![metadata.to_summary()]);
                        }
                    }
                }
            }
        }
    }
    Json(hash_map)
}

#[derive(Responder)]
#[response(content_type = "image/svg+xml")]
struct QrCodeResponse(String);

#[get("/qr?<link>")]
async fn qr(
    link: String,
) -> Result<QrCodeResponse, Status> {

    let code = QrCode::new(link).unwrap();

    let image = code.render::<svg::Color>()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("black"))
        .light_color(svg::Color("white"))
        .build();

    Ok(QrCodeResponse(image))
}

#[get("/qr_html?<link>")]
fn qr_html(link: String, config: &State<Config>) -> content::RawHtml<String> {

    let css_location = match config.dev {
        true => "/js/style.css",
        false => &format!("/js/{}/style.css", VERSION),
    };

    content::RawHtml(format!(indoc!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <link rel="stylesheet" href="{}">
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        <body>
            <div id="app">
                    <div class="primary-card">
                        <div class="content">
                            <header id="primary-header">
                            </header>
                            <div class="everything-feed">
                                <div class="qrcode-frame">
                                    <img class="qrcode" src="/qr?link={}" />
                                </header>
                            </div>

                        </div>
                    </div>
                </div>
            </div>`
        </body>
    </html>
    "#), css_location, link))
    /*
                <div class="primary-card">
                    <div class="content">
                        <header id="primary-header">
                        </header>
                        <div class="everything-feed">
                            <div class="frames">
                                <div class="loader-wrapper">
                                    <div class="loader">
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>`

     */

}

#[get("/boop?<content>")]
fn boop(content: String, remote_addr: IpAddr) -> String {
    println!("Boop: key={} ip={}", content, remote_addr);
    "OK".to_string()
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
        sitemap,
        boop,
        qr,
        qr_html,
        robots,
    ]);

    if std::env::var("ROCKET_ENV").unwrap_or("production".to_string()) == "development"{
        // here we point to the JS and CSS build directories:
        // we only bother with this next bit if we're in dev mode: otherwise we should use include_str! to bundle the files directly into the binary
        println!("Serving JS in dev mode...");
        let dev_ui_location = std::env::var("JS_BUILD_LOCATION").unwrap_or("src/js".to_string());
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
        println!("Serving JS in production mode... (using baked-in JS)");
        app = app.mount("/", routes![js_app, js_css]);
    }

    let services = Services::new();

    app = app.manage(flags);
    app = app.manage(config);
    app = app.manage(services);

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
        println!("  version:    Print the version");
        println!("  init:       Create a new deck in the current directory");
        println!("  new:        Create a new deck in a specified directory");
        println!("  serve:      Start the server");
        std::process::exit(0);
    }
    if args.len() > 1{
        let flags = Flags::from_args(args.clone());

        let arg = &args[1];
        if arg == "version"{
            println!("{}", VERSION);
            std::process::exit(0);
        }
        if arg == "init"{
            init(flags);
            std::process::exit(0);
        }
        if arg == "new"{
            new(flags);
            std::process::exit(0);
        }
        if arg == "status"{
            println!("Status...");
            status(flags);
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
            println!("CardChapter Ministry {}", VERSION);
            println!("Serving...");
        }
    }

    launch_server(flags, config).await
}