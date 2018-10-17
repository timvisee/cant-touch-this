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
                    create_template,
                    delete_template,
                    state,
                    set_state,
                    visualizer,
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

#[get("/api/v1/template/create/<name>/<from>/<to>")]
fn create_template(
    name: String,
    from: usize,
    to: usize,
    gesture_controller: State<Arc<GestureController>>,
) -> Json<bool> {
    Json(gesture_controller.create(name, from, to).is_ok())
}

#[get("/api/v1/template/<id>/delete")]
fn delete_template(id: u32, store: State<Arc<TemplateStore>>) -> Json<bool> {
    store
        .delete(id)
        .expect("failed to delete template and save list");
    Json(true)
}

#[derive(Serialize, Deserialize)]
struct TemplateIndexResponse {
    templates: Vec<GestureTemplate>,
}

#[get("/api/v1/state")]
fn state(gesture_controller: State<Arc<GestureController>>) -> Json<StateResponse> {
    Json(StateResponse {
        state: gesture_controller.state().id(),
    })
}

#[get("/api/v1/state/<state>")]
fn set_state(state: u8, gesture_controller: State<Arc<GestureController>>) -> Json<StateResponse> {
    // Parse the state
    let state = GestureState::from_id(state).expect("failed to parse state ID");

    // Set the state
    gesture_controller.set_state(state);

    // Reset the gesture data if setting the state to normal
    if let GestureState::Normal = state {
        gesture_controller.clear();
    }

    // Respond with the state
    Json(StateResponse { state: state.id() })
}

#[derive(Serialize, Deserialize)]
struct StateResponse {
    state: u8,
}

// TODO: input trim values when saving
// TODO: trim trace before outputting when saving
#[get("/api/v1/visualizer")]
fn visualizer(gesture_controller: State<Arc<GestureController>>) -> Json<LiveTraceResponse> {
    // Get the live data models and detected gestures
    let models = gesture_controller.live_trace();
    let detected = gesture_controller.flush_detected();

    // Respond with the state
    Json(LiveTraceResponse { models, detected })
}

#[derive(Serialize, Deserialize)]
struct LiveTraceResponse {
    models: Vec<Model>,
    detected: Vec<GestureTemplate>,
}
