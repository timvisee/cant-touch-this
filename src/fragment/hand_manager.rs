use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use leap::HandList as SensorHandList;

use super::Hand;
use fragment::FragmentManager;
use types::Model;

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

    /// Process a hand list frame from the sensor.
    pub fn process_sensor_hand_list(
        &self,
        hand_list: SensorHandList,
        fragment_manager: &Arc<FragmentManager>,
    ) {
        // Loop through all hands
        for sensor_hand in hand_list.iter() {
            // Obtain our hand or create a new one
            let hand = self.get(sensor_hand.id()).unwrap_or_else(|| {
                // Create hand in global fragment manager, add it to this manager
                let hand = fragment_manager.create_hand(sensor_hand.id());
                self.add(sensor_hand.id(), hand.clone());
                hand
            });

            // Process the sensor hand
            hand.lock()
                .expect("failed to unlock hand for updating traces")
                .process_sensor_hand(&sensor_hand);
        }

        // Retain hands from the hands map that aren't in view anymore
        self.hands
            .lock()
            .expect("failed to lock hands in fragment manager, for decaying old hands")
            .retain(|&hand_id, _| hand_list.iter().any(|h| h.id() == hand_id));
    }

    /// Get the mutex holding the raw hands.
    /// This may be useful for externally managing the hands hashmap.
    pub fn raw_mutex<'a>(&'a self) -> &'a Mutex<HashMap<i32, Arc<Mutex<Hand>>>> {
        &self.hands
    }

    // TODO: create a method for garbage collecting hands that haven't been updated in a while,
    //       this should be called from all contexts the manager is used, preferreably in some
    //       automated fashion

    // TODO: this is temporary
    pub fn get_live_models(&self) -> Vec<Model> {
        self.hands
            .lock()
            .expect("failed to lock hands manager list")
            .values()
            .filter_map(|hand| hand.lock().expect("failed to lock hand").get_live_model())
            .collect()
    }
}
