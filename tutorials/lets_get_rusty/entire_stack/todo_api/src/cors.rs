// NOTE: There's an existing crate `rocket_cors` that can be used
// to handle CORS in Rocket applications.
// https://docs.rs/rocket_cors/latest/rocket_cors/

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS;

// Fairings: callbacks at launch, liftoff, request, and response time.
// https://api.rocket.rs/master/rocket/fairing/trait.Fairing

// Fairing is an async trait. Implementations of Fairing must be
// decorated with an attribute of #[rocket::async_trait]
#[rocket::async_trait]
impl Fairing for CORS {
    // Required for any fairing implementation.
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    // source: https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // source: https://webprogramming.ninja/2022/08/25/handling-options-requests-in-rust-using-rocket-with-cors/
        if request.method() == Method::Options {
            let body = "";
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), std::io::Cursor::new(body));
            response.set_status(Status::Ok);
        }
    }
}
