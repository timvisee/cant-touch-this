#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

use rocket;
use rocket_contrib::Template;
use rocket_contrib::static_files::StaticFiles;
use std::collections::HashMap;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn start(&self) {
        rocket::ignite()
            .mount("/", routes![index, example])
            .mount("/", StaticFiles::from(["css", "js"]))
            .attach(Template::fairing())
            .launch();
    }
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "world");

    Template::render("index", &context)
}

#[get("/<name>")]
fn example(name: String) -> Template {
    let mut context = HashMap::new();
    context.insert("name", name);

    Template::render("index", &context)
}
