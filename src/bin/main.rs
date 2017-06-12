#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket_contrib;
extern crate rocket;
extern crate poll;
extern crate postgres;
extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use rocket_contrib::{JSON};

// Postgres
use self::poll::*;
use self::poll::models::*;

#[derive(Debug, FromForm)]
struct Question {
    id: i32,
}

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("404", &context)
}

#[get("/answers?<answer>")]
fn answers(answer: Question) -> JSON<Vec<Answer>> {
    println!("connection to db....");
    let conn = cn();
    let mut vec:Vec<Answer> = Vec::new();
    for row in &conn.query("select id, name, value from answers where survies_id = $1", &[&answer.id]).unwrap() {
        let answer = Answer {
            id: row.get(0),
            name: row.get(1),
            value: row.get(2)
        };
        vec.push(answer);
    }
    JSON(vec)
}

#[get("/survies")]
fn survies() -> JSON<Vec<Survie>> {
    println!("connection to db....");
    let conn = cn();
    let mut vec:Vec<Survie> = Vec::new();
    for row in &conn.query("select id, title from survies", &[]).unwrap() {
        let survie = Survie {
            id: row.get(0),
            title: row.get(1)
        };
        vec.push(survie);
    }
    JSON(vec)
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![files, index, survies, answers])
    .catch(errors![not_found])
    .launch();
}
