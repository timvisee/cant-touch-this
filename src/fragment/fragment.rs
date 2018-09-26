use types::{Point3, PointTrace, RotTrace};

use leap::Finger as SensorFinger;

/// A fragment.
// TODO: keep track of the last update time
// TODO: keep track on what data has been recognized
#[derive(Debug)]
pub struct Fragment {
    /// The raw trace, from the sensor.
    raw: PointTrace,

    /// The processed trace used for recognition.
    processed: RotTrace,
}

impl Fragment {
    /// Construct a new fragment with empty traces.
    pub fn new() -> Self {
        Fragment {
            raw: PointTrace::empty(),
            processed: RotTrace::empty(),
        }
    }

    /// Get a mutable reference to the raw point trace in this fragment.
    pub fn raw(&mut self) -> &mut PointTrace {
        &mut self.raw
    }

    /// Get a mutable reference to the processed point trace in this frament.
    pub fn processed(&mut self) -> &mut RotTrace {
        &mut self.processed
    }

    /// Push finger data from a sensor frame on the finger trace.
    /// Then, process the raw data into data we can work with in real-time.
    pub fn process_sensor_finger(&mut self, finger: SensorFinger) {
        self.raw
            .push(Point3::from(finger.stabilized_tip_position()));

        // Calculate the new rotational point based on the new data, add it to the processed trace
        if let Some(x) = self.raw.calc_last_rot_point() {
            self.processed.push(x);
        }
    }
}
