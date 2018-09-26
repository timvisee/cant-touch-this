use std::collections::HashMap;
use std::sync::Arc;

use leap::{FingerType, Hand as SensorHand};

use super::Fragment;
use gesture::GestureController;

/// A hand with traces.
pub struct Hand {
    /// The fingers on this hand, grouped by their finger types.
    fingers: HashMap<FingerType, Fragment>,

    /// Get gesture controller that is used for gesture detection.
    gesture_controller: Arc<GestureController>,
}

impl Hand {
    /// Construct a new hand.
    pub fn new(gesture_controller: Arc<GestureController>) -> Self {
        Hand {
            fingers: HashMap::new(),
            gesture_controller,
        }
    }

    /// Process a sensor hand frame from the sensor.
    pub fn process_sensor_hand(&mut self, hand: &SensorHand) {
        for f in hand.fingers().extended().iter() {
            // Clone the gesture controller for new fragments
            // TODO: only clone for new fragments to improve performance
            let gesture_controller = self.gesture_controller.clone();

            // Process the sensor finger on the fragment, create it if it doesn't exist
            self.fingers
                .entry(f.type_enum())
                .or_insert_with(|| Fragment::new(gesture_controller))
                .process_sensor_finger(f);
        }
    }
}
