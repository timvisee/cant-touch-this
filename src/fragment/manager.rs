use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{Hand, HandManager};

/// A fragment manager.
pub struct FragmentManager {
    /// The hand manager, tracked by the fragment manager.
    hand: HandManager,
}

impl FragmentManager {
    /// Construct a new empty fragment manager.
    pub fn new() -> Self {
        FragmentManager {
            hand: HandManager::new(),
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
            .or_insert_with(|| Arc::new(Mutex::new(Hand::new())))
            .clone()
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while
}
