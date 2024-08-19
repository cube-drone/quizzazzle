#[macro_use]
extern crate rocket;

use std::env;

use rocket::{Build, Rocket};

mod ministry_directory;

fn init(){
    let directory_root = ".";
    let directory = ministry_directory::MinistryDirectory::new(directory_root.to_string());
    directory.init().expect("Failed to initialize directory.");
}

async fn launch_server(_multi: bool) -> Rocket<Build> {

    let mut app = rocket::build();

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

    let mut multi: bool = false;

    if args.len() == 1{
        println!("Help:");
        println!("  init:       Create a new deck in the current directory");
        println!("  status:     Show the status of the deck");
        println!("  diff:       Diff the current deck against the last published deck");
        println!("  serve:      Start the server in single-deck mode, using the deck in the current directory");
        println!("  multi:      Start the server in multi-deck mode");
        println!("  login:      Log-in to a multi-deck server.");
        println!("  publish:    Publish (upload) the current deck to a multi-deck server.");
        std::process::exit(0);
    }
    if args.len() > 1{
        let arg = &args[1];
        if arg == "init"{
            println!("Initializing...");
            init();
            std::process::exit(0);
        }
        if arg == "status"{
            println!("Status...");
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
            multi = true;
        }
    }

    launch_server(multi).await
}