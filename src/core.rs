use std::{io::Result, sync::Arc};

use clap::ArgMatches;
use webbrowser;

use fragment::FragmentManager;
use gesture::GestureController;
use sensor::SensorController;
use store::template::TemplateStore;
#[cfg(feature = "web")]
use web::server::Server;

/// The application core with a global state.
#[allow(dead_code)]
pub struct Core {
    /// The CLI argument matches.
    matches: ArgMatches<'static>,

    /// The sensor controller, handling the sensor data
    ///
    /// This produces 3D point traces.
    sensor_controller: SensorController,

    /// The fragment manager, keeping track of trace fragments.
    fragment_manager: Arc<FragmentManager>,

    /// The gesture controller
    ///
    /// This handles gesture recognition and recording based on rotation traces.
    gesture_controller: Arc<GestureController>,

    /// The gesture template store
    ///
    /// This is used by the gesture controller to match new against.
    store: Arc<TemplateStore>,

    /// The web server
    ///
    /// This is used to launch a web based configuration window for the user.
    #[cfg(feature = "web")]
    server: Server,
}

impl Core {
    /// Construct and initialize the core.
    ///
    /// The CLI arguments matched must be given.
    pub fn new(matches: ArgMatches<'static>) -> Core {
        println!("Initializing core...");

        // Build components in order, depending on each other
        let store = Arc::new(TemplateStore::new());
        let gesture_controller = Arc::new(GestureController::new(store.clone()));
        let fragment_manager = Arc::new(FragmentManager::new(gesture_controller.clone()));

        // TODO: this is temporary
        gesture_controller.set_fragment_manager(fragment_manager.clone());

        Core {
            matches,
            sensor_controller: SensorController::new(fragment_manager.clone()),
            fragment_manager,
            gesture_controller: gesture_controller.clone(),
            store: store.clone(),
            #[cfg(feature = "web")]
            server: Server::new(gesture_controller, store),
        }
    }

    /// Start the core.
    pub fn start(&mut self) -> Result<()> {
        // Load the templates
        self.store.load()?;

        #[cfg(feature = "web")]
        {
            // Open the web configuration page
            // TODO: dynamically obtain URL
            if self.matches.is_present("open") {
                if let Err(err) = webbrowser::open("http://localhost:8000") {
                    eprintln!("Failed to open configuration URL in webbrowser.");
                    eprintln!("{}", err);
                }
            }

            // Start the web server
            self.server.start();
        }

        Ok(())
    }

    /// Stop the core.
    pub fn stop(&mut self) -> Result<()> {
        // Save the templates
        self.store.save()
    }
}

// /// A handle holding a reference to the core.
// pub struct CoreHandle {
//     handle: Arc<Core>,
// }

// impl CoreHandle {
//     /// Construct and initialize a core, return a handle.
//     pub fn new() -> Self {
//         Self::from(Arc::new(Core::new()))
//     }

//     /// Construct a new handle from the given Core wrapped in an Arc.
//     pub fn from(handle: Arc<Core>) -> Self {
//         CoreHandle { handle }
//     }

//     /// Get a reference to the core.
//     pub fn core(&self) -> &Core {
//         self.handle.as_ref()
//     }
// }
