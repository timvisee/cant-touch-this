use std::sync::{Arc, Mutex};

use leap::{Controller as LeapController, FingerType, Listener as LeapListener};

use fragment::{FragmentManager, Hand, HandManager};
use types::{Point3, PointTrace};

/// Structure representing a motion sensor.
pub struct Sensor {
    /// The Leap Motion controller instance.
    controller: LeapController,

    /// A hand manager, which keeps a list of all hands that have recently been tracked by the
    /// sensor, so these are easily accessible when new data arrives.
    hands: Arc<HandManager>,
}

impl Sensor {
    /// Construct a new sensor with the given listener.
    pub fn new(mut listener: SensorListener) -> Self {
        // Create the sensor trace, assin it to the listener
        let hands = Arc::new(HandManager::new());
        listener.set_hand_manager(hands.clone());

        Self {
            controller: LeapController::with_listener(listener),
            hands,
        }
    }
}

/// A sensor listener.
///
/// This listener handles incomming events from the sensor,
/// and processes it's data.
pub struct SensorListener {
    /// A hand manager, which keeps a list of all hands that have recently been tracked by the
    /// sensor, so these are easily accessible when new data arrives.
    hands: Option<Arc<HandManager>>,

    /// The global fragment manager.
    fragment_manager: Arc<FragmentManager>,
}

impl SensorListener {
    /// Construct a new sensor listener.
    pub fn new(fragment_manager: Arc<FragmentManager>) -> Self {
        Self {
            hands: None,
            fragment_manager,
        }
    }

    /// Set the hand manager this listener is working with.
    pub fn set_hand_manager(&mut self, hands: Arc<HandManager>) {
        self.hands = Some(hands);
    }
}

impl LeapListener for SensorListener {
    fn on_frame(&mut self, controller: &LeapController) {
        // Grab a frame from the controller
        let frame = controller.frame();

        println!(
            "Got update from Leap Motion sensor: {} FPS",
            frame.current_fps(),
        );

        // Process hands frame data in hands manager if set
        if let Some(hands) = &self.hands {
            hands.process_sensor_hand_list(
                frame.hands(),
                self.fragment_manager.clone(),
            );
        }

        // // Add the extended index finger position to the trace
        // if let Some(ref trace) = self.trace {
        //     // Get the extended index fingers
        //     let fingers = controller
        //         .frame()
        //         .fingers()
        //         .extended()
        //         .finger_type(FingerType::Index);

        //     // Add the tip points to the trace
        //     for finger in fingers.iter() {
        //         let tip = finger.stabilized_tip_position();

        //         println!("Point: {} , {} , {}", tip.x(), tip.y(), tip.z());

        //         trace
        //             .lock()
        //             .expect("failed to lock sensor trace, cannot extend")
        //             .push(tip.into());
        //     }
        // }
    }

    fn on_connect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor connected");
    }

    fn on_disconnect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor disconnected");
    }
}
