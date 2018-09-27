#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

use std::{collections::HashMap, sync::Arc};

use rocket::{self, State};
use rocket_contrib::{static_files::StaticFiles, Json, Template};

use gesture::GestureController;
use store::TemplateStore;

pub struct Server {
    /// The gesture controller used for managing recordings.
    gesture_controller: Arc<GestureController>,

    /// The template store.
    template_store: Arc<TemplateStore>,
}

impl Server {
    /// Construct a new server with the given `gesture_controller`.
    pub fn new(
        gesture_controller: Arc<GestureController>,
        template_store: Arc<TemplateStore>,
    ) -> Server {
        Server {
            gesture_controller,
            template_store,
        }
    }

    /// Initialize and start the server.
    pub fn start(&self) {
        rocket::ignite()
            .mount("/", routes![index, start_recording])
            .mount("/css", StaticFiles::from("res/static/css"))
            .mount("/js", StaticFiles::from("res/static/js"))
            .manage(self.gesture_controller.clone())
            .manage(self.template_store.clone())
            .attach(Template::fairing())
            .launch();
    }
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}

#[get("/api/v1/start_recording")]
fn start_recording(gesture_controller: State<Arc<GestureController>>) -> Json<RecordResponse> {
    // TODO: move this into a parameter
    let recording = true;

    // Set the recording state in the gesture controller
    gesture_controller.set_recording(recording);

    // Respond with the state
    Json(RecordResponse::new(recording))
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
