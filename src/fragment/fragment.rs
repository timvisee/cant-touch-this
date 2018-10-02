use std::sync::Arc;

use leap::Finger as SensorFinger;

use gesture::GestureController;
use types::{Model, Point3, PointTrace, RotTrace};

/// A fragment.
// TODO: keep track of the last update time
// TODO: keep track on what data has been recognized
#[derive(Debug)]
pub struct Fragment {
    /// The raw trace, from the sensor.
    raw: PointTrace,

    /// The processed trace used for recognition as a model.
    model: Model,

    /// The gesture controller that is used for recongizing gestures.
    gesture_controller: Arc<GestureController>,
}

impl Fragment {
    /// Construct a new fragment with empty traces.
    pub fn new(gesture_controller: Arc<GestureController>) -> Self {
        Fragment {
            raw: PointTrace::empty(),
            model: Model::empty(),
            gesture_controller,
        }
    }

    /// Get a mutable reference to the raw point trace in this fragment.
    pub fn raw(&mut self) -> &mut PointTrace {
        &mut self.raw
    }

    /// Get the processed fragment trace model.
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Push finger data from a sensor frame on the finger trace.
    /// Then, process the raw data into data we can work with in real-time.
    ///
    /// TODO: remove temporary parameter: `process`
    pub fn process_sensor_finger(&mut self, finger: SensorFinger, process: bool) {
        self.raw
            .push(Point3::from(finger.stabilized_tip_position()));

        // Calculate the new rotational point based on the new data,
        // add it to the processed trace
        if let Some(x) = self.raw.to_last_rot_point() {
            self.model.trace_mut().push(x);
        }

        // TODO: do some data normalization (scaling, filtering)

        // Pass the processed data to the gesture controller, for recognition
        if process {
            self.gesture_controller.detect_gesture(&self.model);
        }
    }
}
