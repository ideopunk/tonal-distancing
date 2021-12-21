#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use rocket::data::{Data, ToByteUnit};
use rocket::serde::{json::Json, Serialize};
use serde::ser::{SerializeStruct, Serializer};
use std::fs;

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
    // default if none
    let lookahead = lookahead.unwrap_or(50);

    // default if none
    let stop_words_vec_string = stop_words.unwrap_or_else(|| {
        let pre_vec =
            fs::read_to_string("stop_words.txt").expect("Could not read the stop words file");

        pre_vec
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
    });

    let stop_words_vec_str = stop_words_vec_string
        .iter()
        .map(|word| &**word)
        .collect::<Vec<&str>>();

    let strang = prefile
        .open(2.megabytes())
        .into_string()
        .await
        .context("Failed to open uploaded file")?;

    let word_vec = library::split_text_into_words(strang.into_inner())
        .context("failed to split text into words")?;

    let marked_up_vec = library::mark_up(word_vec, stop_words_vec_str, lookahead);

    let res = library::rebuild_run(marked_up_vec)
        .iter()
        .map(|word| Wrapper(word.clone()))
        .collect();

    Ok(Json(res))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, report])
}
