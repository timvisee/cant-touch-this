use std::sync::Arc;

use leap::HandList as SensorHandList;

use super::HandManager;
use gesture::GestureController;
use types::Model;

/// A fragment manager.
#[derive(Debug)]
pub struct FragmentManager {
    /// The hand manager, tracked by the fragment manager.
    hand: HandManager,

    /// The gesture controller that is used for gesture detection.
    gesture_controller: Arc<GestureController>,
}

impl FragmentManager {
    /// Construct a new empty fragment manager.
    pub fn new(gesture_controller: Arc<GestureController>) -> Self {
        FragmentManager {
            hand: HandManager::new(),
            gesture_controller,
        }
    }

    /// Get the longest model from the fragment manager.
    ///
    /// If no model is available, `None` is returned instead.
    pub fn longest_model(&self) -> Option<Model> {
        self.hand.longest_model()
    }

    // TODO: this is temporary
    pub fn live_models(&self) -> Vec<Model> {
        self.hand.get_live_models()
    }

    /// Clear the hands.
    pub fn clear(&self) {
        self.hand.clear();
    }

    /// Process a hand list frame from the sensor.
    #[inline]
    pub fn process_sensor_hand_list(&self, hand_list: SensorHandList) {
        self.hand.process_sensor_hand_list(hand_list, &self.gesture_controller);
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while
}
