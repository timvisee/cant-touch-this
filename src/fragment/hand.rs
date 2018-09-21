use std::collections::HashMap;

use leap::{FingerType, Hand as SensorHand};

use super::Fragment;

/// A hand with traces.
pub struct Hand {
    /// The fingers on this hand, grouped by their finger types.
    fingers: HashMap<FingerType, Fragment>,
}

impl Hand {
    /// Construct a new hand.
    pub fn new() -> Self {
        Hand {
            fingers: HashMap::new(),
        }
    }

    /// Process a sensor hand frame from the sensor.
    pub fn process_sensor_hand(&mut self, hand: SensorHand) {
        for f in hand.fingers().iter() {
            self.fingers
                .entry(f.type_enum())
                .or_insert_with(|| Fragment::new())
                .process_sensor_finger(f);
        }
    }
}
