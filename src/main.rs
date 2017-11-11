
extern crate iron;
extern crate mount;
extern crate staticfile;
#[macro_use]
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate router;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

mod api;
mod cpuinfo;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("app")));
    mount.mount("/api/v1/", api::create_api());

    Iron::new(mount).http("localhost:3000").unwrap();
}
