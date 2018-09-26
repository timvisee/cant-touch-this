use types::RotTrace;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    /// The model trace
    ///
    /// This will be extended to support multiple traces (for a single hand) in the future.
    trace: RotTrace,
}

impl Model {
    /// Construct a new model.
    pub fn new(trace: RotTrace) -> Self {
        Model { trace }
    }

    /// Construct a new empty model.
    pub fn empty() -> Self {
        Self::new(RotTrace::empty())
    }

    /// Get a mutable reference to the internal rotational trace.
    pub fn trace_mut(&mut self) -> &mut RotTrace {
        &mut self.trace
    }
}
