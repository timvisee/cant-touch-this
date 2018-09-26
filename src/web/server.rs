#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

use rocket;

use rocket_contrib::{Json, Template};
use rocket_contrib::static_files::StaticFiles;

use std::collections::HashMap;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn start(&self) {
        rocket::ignite()
            .mount("/", routes![index, example, start_recording])
            .mount("/css", StaticFiles::from("templates/css"))
            .mount("/js", StaticFiles::from("templates/js"))
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

#[get("/api/v1/start_recording")]
fn start_recording() -> Json<RecordResponse> {
    Json(RecordResponse::new(true))
}

#[derive(Serialize, Deserialize)]
struct RecordResponse {
    started: bool,
}

impl RecordResponse {
    pub fn new(started: bool) -> RecordResponse {
        RecordResponse { started }
    }
}
