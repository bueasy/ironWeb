extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;

use std::path::Path;
use staticfile::Static;
use mount::Mount;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;

fn main() {
    let mut mount = Mount::new();
    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("web/")));

    let mut router = Router::new();
    router.get("/query", query_handler, "query");

    mount.mount("/get", router);

    Iron::new(mount).http("127.0.0.1:3000").unwrap();


    fn query_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }
}
