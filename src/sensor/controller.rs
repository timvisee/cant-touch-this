use super::sensor::{Sensor, SensorListener};

/// This thing should manage controllers, fethcing points
pub struct SensorController {
    /// The leap motion sensor.
    ///
    /// This will be extended in the future for supporting multiple sensors
    sensor: Sensor,
}

impl SensorController {
    /// Construct a sensor controller.
    pub fn new() -> Self {
        Self {
            sensor: Sensor::new(SensorListener::new()),
        }
    }
}
