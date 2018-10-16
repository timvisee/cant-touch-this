use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use leap::HandList as SensorHandList;

use super::Hand;
use types::Model;
use gesture::GestureController;

/// A hand manager.
#[derive(Debug)]
pub struct HandManager {
    /// A hashmap with hands, grouped by their hand ID obtained from the sensor.
    ///
    /// The hashmap and the hands itself are wrapped in a mutex to allow using and mutating these
    /// constructs from any contect by the use of locking.
    hands: Mutex<HashMap<i32, Arc<Mutex<Hand>>>>,
}

impl HandManager {
    /// Construct a new empty hand manager.
    pub fn new() -> Self {
        HandManager {
            hands: Mutex::new(HashMap::new()),
        }
    }

    /// Add the given hand with the given ID to the internal list of hands.
    ///
    /// Note: if a hand with this ID already exists, it is replaced.
    pub fn add(&self, id: i32, hand: Arc<Mutex<Hand>>) {
        self.hands
            .lock()
            .expect("failed to lock hands in frament manager, for adding a new hand")
            .insert(id, hand);
    }

    /// Get a hand from the list.
    /// If no hand exists with this ID, `None` is returned instead.
    pub fn get(&self, id: i32) -> Option<Arc<Mutex<Hand>>> {
        self.hands
            .lock()
            .expect("failed to lock hands in fragment manager, for obtaining a hand")
            .get(&id)
            .cloned()
    }

    /// Find the longest model from the hand list.
    /// If no model exists, `None` is returned instead.
    pub fn longest_model(&self) -> Option<Model> {
        self.hands
            .lock()
            .expect("failed to lock hands in fragment manager, for obtaining longest model")
            .values()
            .filter_map(|h| h.lock().expect("failed to lock hand to find longest model").longest_model())
            .max_by_key(|m| m.len())
    }

    /// Add a hand with the given hand ID.
    ///
    /// Note: if a hand with the given ID already exists, it is returned instead.
    pub fn create_hand(&self, id: i32, gesture_controller: &Arc<GestureController>) -> Arc<Mutex<Hand>> {
        self.hands
            .lock()
            .expect("failed to lock hands manager to add a new hand")
            .entry(id)
            .or_insert_with(|| Arc::new(Mutex::new(Hand::new(gesture_controller.clone()))))
            .clone()
    }

    /// Process a hand list frame from the sensor.
    #[inline]
    pub fn process_sensor_hand_list(&self, hand_list: SensorHandList, guesture_controller: &Arc<GestureController>) {
        // Loop through all hands
        for sensor_hand in hand_list.iter() {
            // Obtain our hand or create a new one
            let hand = self.get(sensor_hand.id()).unwrap_or_else(|| {
                // Create hand in global fragment manager, add it to this manager
                let hand = self.create_hand(sensor_hand.id(), guesture_controller);
                self.add(sensor_hand.id(), hand.clone());
                hand
            });

            // Process the sensor hand
            hand.lock()
                .expect("failed to unlock hand for updating traces")
                .process_sensor_hand(&sensor_hand);
        }

        // Retain hands from the hands map that aren't in view anymore
        self.retain_hands(&hand_list);
    }

    /// Only retain hands in this hand manager that are part of the given `hand_list`.
    /// Other hands are drained from the list.
    pub fn retain_hands(&self, hand_list: &SensorHandList) {
        self.hands
            .lock()
            .expect("failed to lock hands in hand manager, for decaying old hands")
            .retain(|&hand_id, _| hand_list.iter().any(|h| h.id() == hand_id));
    }

    // TODO: this is temporary
    pub fn get_live_models(&self) -> Vec<Model> {
        self.hands
            .lock()
            .expect("failed to lock hands manager list")
            .values()
            .filter_map(|hand| hand.lock().expect("failed to lock hand").get_live_model())
            .collect()
    }

    /// Clear the hands.
    pub fn clear(&self) {
        self.hands
            .lock()
            .expect("failed to lock hands manager list to clear hands")
            .clear();
    }
}
