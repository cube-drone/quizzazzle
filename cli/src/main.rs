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
use rocket::State;
use indoc::indoc; // this is a macro that allows us to write multi-line strings in a more readable way

mod ministry_directory;

const APP_JS: &str = include_str!("../../js/build/out.js");
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

#[get("/js/app.js")]
async fn js_app() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(APP_JS)
}
#[get("/js/style.css")]
async fn js_css() -> content::RawCss<&'static str> {
    content::RawCss(APP_CSS)
}

struct Flags{
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
struct Config{
    server_url: Url,
    site_name: String,
    default_locale: String,
}

impl Config{
    fn from_env() -> Config{
        let server_url = std::env::var("ROCKET_SERVER_URL").unwrap_or("http://localhost:8000".to_string());
        let site_name = std::env::var("ROCKET_SITE_NAME").unwrap_or("Ministry".to_string());
        let default_locale = std::env::var("ROCKET_DEFAULT_LOCALE").unwrap_or("en_US".to_string());
        Config{
            server_url: Url::parse(&server_url).unwrap(),
            site_name,
            default_locale,
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
            <script src="/js/app.js"></script>
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
fn index(flags: &State<Flags>, config: &State<Config>) -> content::RawHtml<String> {
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


async fn launch_server(flags: Flags, config: Config) -> Rocket<Build> {

    let mut app = rocket::build();

    app = app.mount("/", routes![index]);

    if std::env::var("ROCKET_ENV").unwrap_or("development".to_string()) == "development"{
        // here we point to the JS and CSS build directories:
        // we only bother with this next bit if we're in dev mode: otherwise we should use include_str! to bundle the files directly into the binary
        let dev_ui_location = std::env::var("JS_BUILD_LOCATION").unwrap_or("../../js/build".to_string());
        //if location exists:
        match Path::new(&dev_ui_location).exists(){
            true => {
                println!("Serving from: {}", dev_ui_location);
                app = app.mount("/js", FileServer::from(dev_ui_location));
            },
            false => {
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

    // home is where "/" lives.
    app = home::routes::mount_routes(app);
    // auth: login, registration, that sort of stuff
    app = auth::routes::mount_routes(app);
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