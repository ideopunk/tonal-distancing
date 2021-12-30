#[macro_use]
extern crate rocket;

use library::{definitions, functions};
use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Header, Status};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::{json, Value};
use rocket::serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    // <'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>)
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // fn on_response<'a>(&self, _request: &'a Request<'_>, response: &mut Response<'a>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

struct Wrapper(definitions::Run);

impl Serialize for Wrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Run", 2)?;
        s.serialize_field("text", &self.0.text)?;
        s.serialize_field("repeated", &self.0.repeated)?;
        s.end()
    }
}

#[derive(Debug)]
struct ApiResponse {
    json: Value,
    status: Status,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'o> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post(
    "/report?<lookahead>&<stop_words>",
    format = "plain",
    data = "<prefile>"
)]
async fn report(
    lookahead: Option<usize>,
    stop_words: Option<Vec<String>>,
    prefile: Data<'_>,
) -> ApiResponse {
    // get content
    let content = prefile.open(2.megabytes()).into_string().await;

    let content = match content {
        Ok(c) => c,
        Err(_) => {
            return ApiResponse {
                json: json!("Failed to open uploaded file"),
                status: Status { code: 404 },
            }
        }
    };
    let content = content.into_inner();

    // println!("{}", content);

    // get look ahead
    let lookahead = lookahead.unwrap_or(50);

    // get stop words
    let stop_words = match stop_words {
        Some(sw) => {
            if sw.len() > 0 {
                functions::get_stop_words(Some(definitions::Source::Raw(sw.join(""))))
            } else {
                functions::get_stop_words(None)
            }
        }
        None => functions::get_stop_words(None),
    };

    // get our report
    let res = functions::tell_you_how_bad(
        content,
        lookahead,
        stop_words,
        definitions::ResponseType::Raw,
    );

    let res = match res {
        Ok(resp) => resp,
        Err(_) => {
            return ApiResponse {
                json: json!("Failed to process content"),
                status: Status { code: 500 },
            }
        }
    };

    match res {
        definitions::Response::VecOfRuns(val) => {
            let wrapped = val
                .iter()
                .map(|word| Wrapper(word.clone()))
                .collect::<Vec<Wrapper>>();
            // Ok(Json(wrapped))
            ApiResponse {
                json: json!(wrapped),
                status: Status { code: 200 },
            }
        }
        _ => ApiResponse {
            json: json!("act of god"),
            status: Status { code: 500 },
        },
    }
}

#[options("/report?<_lookahead>&<_stop_words>")]
fn report_preflight(
    _lookahead: Option<usize>,
    _stop_words: Option<Vec<String>>,
) -> response::status::NoContent {
    response::status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, report, report_preflight])
}
