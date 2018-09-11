use leap::Controller as LeapController;

/// This thing should manage controllers, fethcing points
pub struct SensorController { }

impl SensorController {
    /// Construct a sensor controller.
    pub fn new() -> Self {
        Self {}
    }
}

/// Structure representing a motion sensor.
pub struct Sensor {
    controller: LeapController,
}

impl Sensor {
    /// Construct a new sensor.
    pub fn new() -> Self {
        Self {
            controller: LeapController::new(),
        }
    }
}

// TODO: sensor struct
