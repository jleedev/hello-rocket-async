#[macro_use]
extern crate rocket;

use rocket::http::uri::Origin;
use rocket::http::{HeaderMap, Method};
use rocket::request::{FromRequest, Outcome};
use rocket::tokio::time::{delay_for, Duration};
use rocket::Request;

#[get("/")]
async fn index() -> &'static str {
    r#"GET /delay/<seconds>
GET /env
GET /echo
"#
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    delay_for(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds\n", seconds)
}

#[get("/env")]
async fn env() -> String {
    std::env::vars()
        .map(|(key, value)| format!("{}={}\n", key, value))
        .collect()
}

struct AllHeaders<'a, 'r>(&'a HeaderMap<'r>);
#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AllHeaders<'a, 'r> {
    type Error = std::convert::Infallible;
    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self(request.headers()))
    }
}

#[get("/echo")]
async fn echo(all_headers: AllHeaders<'_, '_>, origin: &Origin<'_>, method: Method) -> String {
    let AllHeaders(hm) = all_headers;
    let headers: String = hm.iter().map(|h| format!("{}\n", h)).collect();
    format!("{} {}\n{}", method, origin, headers)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, delay, env, echo])
}
