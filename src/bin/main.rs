#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate poll;
extern crate diesel;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::Template;

// Diesel
use self::poll::*;
use self::poll::models::*;
use self::diesel::prelude::*;

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("404", &context)
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![files, index])
    .catch(errors![not_found])
    .launch();
}
