#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

use std::{collections::HashMap, io, sync::Arc};

use rocket::{self, data::FromData, State};
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
            .mount("/", routes![index, template_index, record, set_record])
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

#[get("/api/v1/template")]
fn template_index(store: State<Arc<TemplateStore>>) -> Json<TemplateIndexResponse> {
    Json(TemplateIndexResponse {
        templates: store.names(),
    })
}

#[derive(Serialize, Deserialize)]
struct TemplateIndexResponse {
    templates: Vec<String>,
}

#[get("/api/v1/record")]
fn record(gesture_controller: State<Arc<GestureController>>) -> Json<RecordResponse> {
    Json(RecordResponse {
        recording: gesture_controller.recording(),
    })
}

#[get("/api/v1/record/<record>")]
fn set_record(
    record: bool,
    gesture_controller: State<Arc<GestureController>>,
) -> Json<RecordResponse> {
    // Set the recording state in the gesture controller
    gesture_controller.set_recording(record);

    // Respond with the state
    Json(RecordResponse { recording: record })
}

#[derive(Serialize, Deserialize)]
struct RecordResponse {
    recording: bool,
}
