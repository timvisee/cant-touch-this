use beautifier::Beautifier;
use gesture::controller::GestureController;
use sensor::controller::SensorController;
use store::template::TemplateStore;
use web::server::Server;

pub struct Core {
    /// The sensor controller, handling the sensor data
    ///
    /// This produces 3D point traces.
    sensor_controller: SensorController,

    /// The sensor data beautifier
    ///
    /// This beautifies 3D point traces and produces a rotation trace.
    beautifier: Beautifier,

    /// The gesture controller
    ///
    /// This handles gesture recognition and recording based on rotation traces.
    gesture_controller: GestureController,

    /// The gesture template store
    ///
    /// This is used by the gesture controller to match new against.
    store: TemplateStore,

    /// The web server
    ///
    /// This is used to launch a web based configuration window for the user.
    server: Server,
}

impl Core {
    /// Construct and initialize the core.
    pub fn new() -> Core {
        println!("Initializing core...");

        Core {
            sensor_controller: SensorController::new(),
            beautifier: Beautifier::new(),
            gesture_controller: GestureController::new(),
            store: TemplateStore::new(),
            server: Server::new(),
        }
    }
}
