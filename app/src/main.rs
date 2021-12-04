#[macro_use]
extern crate rocket;

use rocket::Data;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/report", data = "<file>")]
fn report(file: Data) -> &'static str {
    "report"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, report])
}
