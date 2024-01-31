
use rocket::Request;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Response;
use rand::Rng;


const POWERED_BY: [&str;25] = [
    "PHP/1.0.4",
    "ASP.NET.NET",
    "JavaServer PageMakers",
    "IBM WebSphere Application Server",
    "OS/2 Warp 4.5.2",
    "Plan 9 From Bell Labs",
    "Borland C++ 5.5.1",
    "Microsoft Visual Basic 6.0",
    "Mavis Bacon Teaches Operating Systems",
    "Multiple Pythons/2.7.0",
    "Ruby on Ramps",
    "tootly tootly toot, tootly tootly toot, tootly tootly toot, tootly tootly toot, I'm a trumpet",
    "off-hours development of an honestly, pretty baffling product",
    "who would want this?",
    "literally just the one guy",
    "(the server hurts itself in its confusion)",
    "a whole room full of small wheels all being turned by tiny adorable rodents",
    "beans, lots of beans, lots of beans lots of beans, yeah, beans",
    "ancient plankton, left in the earth for millions of years, and then refined into a flammable liquid",
    "human suffering, specifically my suffering, specifically my suffering at the hands of this project, specifically my suffering at the hands of this project that I am doing for free",
    "AMD Ryzen 9 5950X 16-Core Processor",
    "Haskell - ha ha, no, I am joking, I didn't think anybody would believe that, ha ha, Haskell, ha ha",
    "CoffeeScript",
    "MediaWiki",
    "Esperanto",
];

fn get_random_powered_by() -> &'static str {
    // return a random element from the POWERED_BY array
    POWERED_BY[rand::thread_rng().gen_range(0..POWERED_BY.len())]
}


/// Fairing for sassy nonsense.
pub struct PoweredBy;

#[rocket::async_trait]
impl Fairing for PoweredBy {
    fn info(&self) -> Info {
        Info {
            name: "X-Powered-By",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Adds a header to the response indicating how long the server took to
    /// process the request.
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("X-Powered-By", get_random_powered_by());
    }
}