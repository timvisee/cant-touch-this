use std::sync::{Arc, Mutex};

use types::Model;
use super::{Hand, HandManager};
use gesture::GestureController;

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

    // TODO: this is temporary
    pub fn get_live_models(&self) -> Vec<Model> {
        self.hand.get_live_models()
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while
}
