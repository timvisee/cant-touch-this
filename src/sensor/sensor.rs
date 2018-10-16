use std::sync::Arc;

use leap::{Controller as LeapController, Listener as LeapListener};

use fragment::FragmentManager;

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
    /// The global fragment manager.
    fragment_manager: Arc<FragmentManager>,
}

impl SensorListener {
    /// Construct a new sensor listener.
    pub fn new(fragment_manager: Arc<FragmentManager>) -> Self {
        Self { fragment_manager }
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
        self.fragment_manager.process_sensor_hand_list(frame.hands());
    }

    fn on_connect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor connected");
    }

    fn on_disconnect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor disconnected");
    }
}
