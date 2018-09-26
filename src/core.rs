use std::sync::Arc;

use beautifier::Beautifier;
use fragment::FragmentManager;
use gesture::GestureController;
use sensor::SensorController;
use store::template::TemplateStore;
#[cfg(feature = "web")]
use web::server::Server;

/// The application core with a global state.
#[allow(dead_code)]
pub struct Core {
    /// The sensor controller, handling the sensor data
    ///
    /// This produces 3D point traces.
    sensor_controller: SensorController,

    /// The fragment manager, keeping track of trace fragments.
    fragment_manager: Arc<FragmentManager>,

    /// The sensor data beautifier
    ///
    /// This beautifies 3D point traces and produces a rotation trace.
    /// TODO: is this obsolete?
    beautifier: Beautifier,

    /// The gesture controller
    ///
    /// This handles gesture recognition and recording based on rotation traces.
    gesture_controller: Arc<GestureController>,

    /// The gesture template store
    ///
    /// This is used by the gesture controller to match new against.
    store: TemplateStore,

    /// The web server
    ///
    /// This is used to launch a web based configuration window for the user.
    #[cfg(feature = "web")]
    server: Server,
}

impl Core {
    /// Construct and initialize the core.
    pub fn new() -> Core {
        println!("Initializing core...");

        // Build the fragment manager
        let fragment_manager = Arc::new(FragmentManager::new());

        Core {
            sensor_controller: SensorController::new(fragment_manager.clone()),
            fragment_manager,
            beautifier: Beautifier::new(),
            gesture_controller: Arc::new(GestureController::new()),
            store: TemplateStore::new(),
            #[cfg(feature = "web")]
            server: Server::new(),
        }
    }

    /// Start the core.
    pub fn start(&mut self) {
        // Load the templates
        self.store.load();

        // Start the web server
        #[cfg(feature = "web")]
        self.server.start();
    }

    /// Stop the core.
    pub fn stop(&mut self) {
        // Save the templates
        self.store.save();
    }
}

/// A handle holding a reference to the core.
pub struct CoreHandle {
    handle: Arc<Core>,
}

impl CoreHandle {
    /// Construct and initialize a core, return a handle.
    pub fn new() -> Self {
        Self::from(Arc::new(Core::new()))
    }

    /// Construct a new handle from the given Core wrapped in an Arc.
    pub fn from(handle: Arc<Core>) -> Self {
        CoreHandle { handle }
    }

    /// Get a reference to the core.
    pub fn core(&self) -> &Core {
        self.handle.as_ref()
    }
}
