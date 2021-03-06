use std::sync::Arc;

use leap::Finger as SensorFinger;

use gesture::GestureController;
use types::{Model, Point3, PointTrace};

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

    /// Get the processed fragment trace model.
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Push finger data from a sensor frame on the finger trace.
    /// Then, process the raw data into data we can work with in real-time.
    ///
    /// TODO: remove temporary parameter `process`
    pub fn process_sensor_finger(&mut self, finger: &SensorFinger, process: bool) {
        // Add the point to the trace
        if self.gesture_controller.state().should_track() {
            self.raw
                .push(Point3::from(finger.stabilized_tip_position()));

            // TODO: currently resampling/recalculating whole trace,
            // TODO: reimplement to only sample/calculate the new point
            // // Calculate the new rotational point based on the new data,
            // // add it to the processed trace
            // if let Some(x) = self.raw.to_last_rot_point() {
            //     self.model.trace_mut().push(x);
            // }
            *self.model.trace_mut() = self.raw.to_rot_trace(true);

            // TODO: do some data normalization (scaling, filtering)
        }

        // Pass the processed data to the gesture controller, for recognition
        if process && self.gesture_controller.state().should_detect() {
            // TODO: do not clone here
            self.gesture_controller.clone().detect_gesture(self);
        }
    }

    /// Clear most of the trace, except for the last few (newest) points as
    /// specified in `config::recognition::KEEP_POINTS`.
    ///
    /// The the number of current points is the same or less than
    /// `KEEP_POINTS`, no points are removed from the trace.
    pub fn clear_most(&mut self) {
        self.raw.clear_most();
        self.model.clear();
    }
}
