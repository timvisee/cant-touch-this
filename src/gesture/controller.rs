use types::Model;

/// Gesture controller, for recognition and recording
///
/// TODO: define the following in sub structures,
/// TODO: this should be a state machine:
/// TODO: build the recognition part, which compares it to a set of templates
/// TODO: build the recording part
#[derive(Debug)]
pub struct GestureController {}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new() -> Self {
        Self {}
    }

    /// Attempt to detect gestures in the given collected model.
    pub fn detect_gesture(&self, model: &Model) {
        // TODO: gesture detection logic for model here
        // TODO: pass the model laong to the template store

        println!("DEBUG: processing model");
    }
}
