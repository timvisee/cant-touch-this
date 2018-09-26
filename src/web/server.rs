#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

use std::{
    collections::HashMap,
    sync::Arc,
};

use rocket;
use rocket_contrib::{
    Json,
    Template,
    static_files::StaticFiles,
};

use gesture::GestureController;

pub struct Server {
    /// The gesture controller used for managing recordings.
    gesture_controller: Arc<GestureController>,
}

impl Server {
    /// Construct a new server with the given `gesture_controller`.
    pub fn new(gesture_controller: Arc<GestureController>) -> Server {
        Server { gesture_controller }
    }

    /// Initialize and start the server.
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
