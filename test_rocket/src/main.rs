#![feature(proc_macro_hygiene, decl_macro)]

use rocket;
use rocket::response::Stream;
use rocket::*;
use std::fs::File;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/stream")]
fn stream() -> Result<Stream<File>, std::io::Error> {
    let f = File::open("test.txt")?;
    let s = Stream::from(f);
    Ok(s)
}

fn main() {
    rocket::ignite().mount("/", routes![index, stream]).launch();
}
