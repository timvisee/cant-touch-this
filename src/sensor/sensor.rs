use std::sync::{Arc, Mutex};

use leap::{Controller as LeapController, FingerType, Listener as LeapListener};

use types::{Point3, PointTrace};

/// Structure representing a motion sensor.
pub struct Sensor {
    /// The Leap Motion controller instance.
    controller: LeapController,

    /// The trace from the sensor.
    ///
    /// Slowly being extended with incomming data from the sensor through the sensor listener.
    /// This should be extended to support multiple traces in the future.
    trace: Arc<Mutex<PointTrace>>,
}

impl Sensor {
    /// Construct a new sensor with the given listener.
    pub fn new(mut listener: SensorListener) -> Self {
        // Create the sensor trace, assin it to the listener
        let trace = Arc::new(Mutex::new(PointTrace::empty()));
        listener.set_trace(trace.clone());

        Self {
            controller: LeapController::with_listener(listener),
            trace,
        }
    }
}

/// A sensor listener.
///
/// This listener handles incomming events from the sensor,
/// and processes it's data.
pub struct SensorListener {
    // TODO: add a reference to the trace, somehow, this sensor is working with
    trace: Option<Arc<Mutex<PointTrace>>>,
}

impl SensorListener {
    /// Construct a new sensor listener.
    pub fn new() -> Self {
        Self { trace: None }
    }

    /// Set the trace this listener is working with.
    pub fn set_trace(&mut self, trace: Arc<Mutex<PointTrace>>) {
        self.trace = Some(trace);
    }
}

impl LeapListener for SensorListener {
    fn on_frame(&mut self, controller: &LeapController) {
        println!(
            "Got update from Leap Motion sensor: {} FPS",
            controller.frame().current_fps()
        );

        // Add the extended index finger position to the trace
        if let Some(ref trace) = self.trace {
            // Get the extended index fingers
            let fingers = controller
                .frame()
                .fingers()
                .extended()
                .finger_type(FingerType::Index);

            // Add the tip points to the trace
            for finger in fingers.iter() {
                let tip = finger.stabilized_tip_position();

                println!("Point: {} , {} , {}", tip.x(), tip.y(), tip.z());

                trace
                    .lock()
                    .expect("failed to lock sensor trace, cannot extend")
                    .push(tip.into());
            }
        }
    }

    fn on_connect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor connected");
    }

    fn on_disconnect(&mut self, _: &LeapController) {
        println!("Leap Motion sensor disconnected");
    }
}
