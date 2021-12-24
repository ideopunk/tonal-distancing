#[macro_use]
extern crate rocket;

use library::{definitions, functions};
use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::{json, Value};
use rocket::serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

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

    // get look ahead
    let lookahead = lookahead.unwrap_or(50);

    // get stop words
    let stop_words = match stop_words {
        Some(sw) => functions::get_stop_words(Some(definitions::Source::Raw(sw.join("")))),
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

    let uh = match res {
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
    };

    uh
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, report])
}
