#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate rocket_contrib;
extern crate rocket;
extern crate poll;
extern crate postgres;
extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use rocket_contrib::{JSON, Value};
use std::collections::HashMap;

// Postgres
use self::poll::*;
use self::poll::models::*;

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("404", &context)
}

#[get("/survies")]
fn survies() -> JSON<Value> {
    println!("connection to db....");
    let conn = cn();
    /*let mut quests: HashMap<&str, String>;
    quests = HashMap::new();*/
    let mut survies: HashMap<String, Survie>;
    survies = HashMap::new();
    for row in &conn.query("SELECT id, title FROM survies", &[]).unwrap() {
        let id: i32 = row.get(0);
        let survie = Survie {
            id: row.get(0),
            title: row.get(1)
        };
        survies.insert(id.to_string(), survie);
        /*print!("r {:?}", row);
        let id: i32 = row.get(0);
        quests.insert("id", id.to_string());
        quests.insert("title", row.get(1));*/
    }
    JSON(json!(survies))
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![files, index, survies])
    .catch(errors![not_found])
    .launch();
}
