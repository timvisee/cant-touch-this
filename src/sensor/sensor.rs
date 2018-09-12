use leap::{Controller as LeapController, Listener as LeapListener};

/// Structure representing a motion sensor.
pub struct Sensor {
    controller: LeapController,
}

impl Sensor {
    /// Construct a new sensor.
    pub fn new(listener: SensorListener) -> Self {
        Self {
            controller: LeapController::with_listener(listener),
        }
    }
}

/// A sensor listener.
pub struct SensorListener {}

impl SensorListener {
    /// Construct a new sensor listener.
    pub fn new() -> Self {
        Self {}
    }
}

impl LeapListener for SensorListener {
    fn on_frame(&mut self, controller: &LeapController) {
        println!(
            "Got update from Leap Motion sensor: {} FPS",
            controller.frame().current_fps()
        );
    }

    fn on_connect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor connected");
    }

    fn on_disconnect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor disconnected");
    }
}
