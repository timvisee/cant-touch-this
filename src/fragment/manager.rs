use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::Hand;

/// A fragment manager.
pub struct FragmentManager {
    /// A hashmap with hands, grouped by their hand ID obtained from the sensor.
    hands: Mutex<HashMap<i32, Arc<Mutex<Hand>>>>,
}

impl FragmentManager {
    /// Construct a new empty fragment manager.
    pub fn new() -> Self {
        FragmentManager {
            hands: Mutex::new(HashMap::new()),
        }
    }

    /// Add a hand with the given hand ID.
    /// If a hand with the given ID already exists, it is returned instead.
    pub fn add_hand(&self, id: i32) -> Arc<Mutex<Hand>> {
        self.hands
            .lock()
            .expect("failed to lock hands in frament manager, for adding a new hand")
            .entry(id)
            .or_insert_with(|| Arc::new(Mutex::new(Hand::new())))
            .clone()
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while
}
