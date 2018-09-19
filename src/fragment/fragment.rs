use types::{PointTrace, RotTrace};

/// A fragment.
// TODO: keep track of the last update time
// TODO: keep track on what data has been recognized
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
}
