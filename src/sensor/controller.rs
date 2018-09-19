use std::sync::Arc;

use fragment::FragmentManager;

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
    ///
    /// As the sensor might use the fragment manager for tracking data,
    /// a refrence to it must be given.
    pub fn new(fragment_manager: Arc<FragmentManager>) -> Self {
        Self {
            sensor: Sensor::new(SensorListener::new(fragment_manager)),
        }
    }
}
