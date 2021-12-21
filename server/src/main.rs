#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::tokio;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/report", format = "plain", data = "<prefile>")]
async fn report(prefile: Data<'_>) -> std::io::Result<()> {
    prefile
        .open(2.megabytes())
        .stream_to(tokio::io::stdout())
        .await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, report])
}
