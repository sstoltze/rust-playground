// From https://christine.website/blog/how-i-start-rust-2020-03-15
#![feature(proc_macro_hygiene, decl_macro)] // Nightly-only language features needed by rocket

// Macros from rocket
#[macro_use]
extern crate rocket;

// Import OpenAPI macros
#[macro_use]
extern crate rocket_okapi;

use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{JsonSchema, OpenApiError};
use serde::*;

#[derive(Serialize, JsonSchema, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

// Create a route /hostinfo that returns information about the host serving the page
#[openapi]
#[get("/hostinfo")]
fn hostinfo() -> Result<Json<HostInfo>, OpenApiError> {
    match gethostname::gethostname().into_string() {
        Ok(hostname) => Ok(Json(HostInfo {
            hostname,
            pid: std::process::id(),
            uptime: psutil::host::uptime().unwrap().as_secs(),
        })),
        Err(_) => Err(OpenApiError::new("hostname does not parse as UTF-8".to_string())),
    }
}

// Create route / that returns "Hello, World!"
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index, hostinfo])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../openapi.json".to_owned()),
                urls: None,
            }),
        )
        .launch();
}

#[cfg(test)] // only compile when unit testing is requested
mod tests {
    use super::*; // Modules have own scope, so explicitly import parent
    use rocket::http::Status;
    use rocket::local::*;

    #[test]
    fn test_index() {
        // create rocket instance to test
        let rkt = rocket::ignite().mount("/", routes![index]);

        // create a http client bound to this rocket instance
        let client = Client::new(rkt).expect("valid rocket");

        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);

        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
