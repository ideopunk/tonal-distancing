#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use rocket::data::{Data, ToByteUnit};
use rocket::serde::{json::Json, Serialize};
use serde::ser::{SerializeStruct, Serializer};

struct Wrapper(library::Run);

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
) -> Result<Json<Vec<Wrapper>>, rocket::response::Debug<anyhow::Error>> {
    // get content
    let content = prefile
        .open(2.megabytes())
        .into_string()
        .await
        .context("Failed to open uploaded file")?
        .into_inner();

    // get look ahead
    let lookahead = lookahead.unwrap_or(50);

    // get stop words
    let stop_words = library::get_stop_words_from_string(stop_words);

    // get our report
    let res = library::tell_you_how_bad(content, lookahead, stop_words, library::ResponseType::Raw)
        .context("Failed to process content")?;

    let uh = match res {
        library::Response::VecOfRuns(val) => {
            let wrapped = val.iter().map(|word| Wrapper(word.clone())).collect();
            Ok(Json(wrapped))
        }
        _ => Err(rocket::response::Debug(anyhow::Error::new(
            library::TonalDistanceError::UhhhError,
        ))),
    };

    uh
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, report])
}
