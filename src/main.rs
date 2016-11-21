extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("web/")));
 

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}