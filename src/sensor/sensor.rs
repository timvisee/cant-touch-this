use leap::Controller as LeapController;

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
