use std::sync::{Arc, Mutex};

use super::{Hand, HandManager};
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

    /// Add a hand with the given hand ID.
    ///
    /// Note: if a hand with the given ID already exists, it is returned instead.
    pub fn create_hand(&self, id: i32) -> Arc<Mutex<Hand>> {
        self.hand
            .raw_mutex()
            .lock()
            .expect("failed to lock hands manager to add a new hand")
            .entry(id)
            .or_insert_with(|| Arc::new(Mutex::new(Hand::new(self.gesture_controller.clone()))))
            .clone()
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

    /// Get the hand manager.
    pub fn hand_manager(&self) -> &HandManager {
        &self.hand
    }

    /// Clear the hands.
    pub fn clear(&self) {
        self.hand.clear();
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while
}
