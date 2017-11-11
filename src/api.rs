
use router::Router;

use iron::prelude::*;
use iron::{Request, Response, IronResult, IronError};
use iron::status;
use iron::headers::ContentType;

use cpuinfo;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn cpuinfo_handler(_: &mut Request) -> IronResult<Response> {
    cpuinfo::cpuinfo()
        .and_then( |value| {
            Ok(Response::with((ContentType::json().0, status::Ok, value.to_string())))
        })
        .map_err( |e| {IronError::new(e, status::InternalServerError)})

}

pub fn create_api() -> Router {
    router!(
        root: get "/" => handler,
        cpuinfo: get "/cpuinfo" => cpuinfo_handler
    )

}
