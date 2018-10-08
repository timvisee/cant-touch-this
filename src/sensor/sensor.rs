use std::sync::Arc;

use leap::{Controller as LeapController, Listener as LeapListener};

use fragment::{FragmentManager, HandManager};

/// Structure representing a motion sensor.
pub struct Sensor {
    /// The Leap Motion controller instance.
    _controller: LeapController,
}

impl Sensor {
    /// Construct a new sensor with the given listener.
    pub fn new(listener: SensorListener) -> Self {
        Self {
            _controller: LeapController::with_listener(listener),
        }
    }
}

/// A sensor listener.
///
/// This listener handles incomming events from the sensor,
/// and processes it's data.
pub struct SensorListener {
    /// A hand manager, which keeps a list of all hands that have recently been tracked by the
    /// sensor, so these are easily accessible when new data arrives.
    hands: HandManager,

    /// The global fragment manager.
    fragment_manager: Arc<FragmentManager>,
}

impl SensorListener {
    /// Construct a new sensor listener.
    pub fn new(fragment_manager: Arc<FragmentManager>) -> Self {
        Self {
            hands: HandManager::new(),
            fragment_manager,
        }
    }
}

impl LeapListener for SensorListener {
    fn on_frame(&mut self, controller: &LeapController) {
        // Grab a frame from the controller
        let frame = controller.frame();

        // println!(
        //     "Got update from Leap Motion sensor: {} FPS",
        //     frame.current_fps(),
        // );

        // Process the hand frame data in the hand manager
        self.hands
            .process_sensor_hand_list(frame.hands(), &self.fragment_manager);
    }

    fn on_connect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor connected");
    }

    fn on_disconnect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor disconnected");
    }
}
