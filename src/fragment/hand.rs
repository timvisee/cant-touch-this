use std::collections::HashMap;
use std::sync::Arc;

use leap::{FingerType, Hand as SensorHand};

use gesture::GestureController;
use super::Fragment;
use types::Model;

/// A hand with traces.
#[derive(Debug)]
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
        // TODO: do not only walk through the extended fingers here
        for f in hand.fingers().extended().iter() {
            // Clone the gesture controller for new fragments
            // TODO: only clone for new fragments to improve performance
            let gesture_controller = self.gesture_controller.clone();

            // Only process extended index fingers
            // TODO: process all fingers after debugging
            let process = f.type_enum() == FingerType::Index && f.is_extended();

            // Process the sensor finger on the fragment, create it if it doesn't exist
            self.fingers
                .entry(f.type_enum())
                .or_insert_with(|| Fragment::new(gesture_controller))
                .process_sensor_finger(f, process);
        }
    }

    // TODO: this is temporary
    pub fn get_live_model(&self) -> Option<Model> {
        // TODO: do not clone here
        self.fingers
            .get(&FingerType::Index)
            .map(|fragment| fragment.model().clone())
    }
}
