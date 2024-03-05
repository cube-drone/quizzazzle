use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use std::net::IpAddr;
use moka::future::Cache;
use std::sync::Arc;
use rocket::http::uri::Origin;
use rocket::http::Method::{Get, Post, Put, Delete};


/// Nobody should be able to hit _any_ endpoint on the server too aggressively.
pub struct RateLimit{
    pub ip_limit: Arc<Cache<IpAddr, bool>>,
}

#[rocket::async_trait]
impl Fairing for RateLimit {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limit",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Adds a header to the response indicating how long the server took to
    /// process the request.
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        println!("{}", req.uri().to_string());
        match req.method(){
            Get => {
                // i don't think we should rate limit GET requests, yet
            }
            Post | Put | Delete => {
                match req.client_ip(){
                    Some(ip) => {
                        let bounce = self.ip_limit.get(&ip).await;

                        match bounce {
                            Some(seen_before) => {
                                if seen_before{
                                    req.set_uri(Origin::parse("/rate").expect("Invalid URI"));
                                }
                                else{
                                    self.ip_limit.insert(ip.clone(), true).await;
                                }
                            }
                            None => {
                                self.ip_limit.insert(ip.clone(), true).await;
                            }
                        }
                    }
                    None => {
                        req.set_uri(Origin::parse("/rate").expect("Invalid URI"));
                    }
                }
            }
            _ => {
                return;
            }
        }
    }
}