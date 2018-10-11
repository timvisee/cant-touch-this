use std::{collections::HashMap, sync::Arc};

use rocket::{self, State};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};

use gesture::{GestureController, GestureState};
use store::TemplateStore;
use types::{Model, Template as GestureTemplate};

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
            .mount(
                "/",
                routes![
                    index,
                    template_index,
                    delete_template,
                    save_template,
                    record,
                    set_record,
                    visualizer_points
                ],
            )
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
        templates: store.to_templates(),
    })
}

#[get("/api/v1/template/<id>/delete")]
fn delete_template(id: u32, store: State<Arc<TemplateStore>>) -> Json<bool> {
    store.delete(id).expect("failed to delete template and save list");
    Json(true)
}

#[get("/api/v1/template/save")]
fn save_template() -> &'static str {
    // TODO: Pass data into this method, then save it to the template file
    "hello, world"
}

#[derive(Serialize, Deserialize)]
struct TemplateIndexResponse {
    templates: Vec<GestureTemplate>,
}

#[get("/api/v1/state")]
fn record(gesture_controller: State<Arc<GestureController>>) -> Json<RecordResponse> {
    Json(RecordResponse {
        state: gesture_controller.state().id(),
    })
}

#[get("/api/v1/state/<state>")]
fn set_record(
    state: u8,
    gesture_controller: State<Arc<GestureController>>,
) -> Json<RecordResponse> {
    // Parse the state
    let state = GestureState::from_id(state).expect("failed to parse state ID");

    // Set the state
    gesture_controller.set_state(state);

    // Respond with the state
    Json(RecordResponse { state: state.id() })
}

#[derive(Serialize, Deserialize)]
struct RecordResponse {
    state: u8,
}

#[get("/api/v1/visualizer/points")]
fn visualizer_points(gesture_controller: State<Arc<GestureController>>) -> Json<LiveTraceResponse> {
    // Get the live data models
    let models = gesture_controller.get_live_trace();

    // Respond with the state
    Json(LiveTraceResponse { models })
}

#[derive(Serialize, Deserialize)]
struct LiveTraceResponse {
    models: Vec<Model>,
}
