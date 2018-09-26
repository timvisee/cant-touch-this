use std::sync::Arc;

use store::TemplateStore;
use types::Model;

/// Gesture controller, for recognition and recording
///
/// TODO: define the following in sub structures,
/// TODO: this should be a state machine:
/// TODO: build the recognition part, which compares it to a set of templates
/// TODO: build the recording part
#[derive(Debug)]
pub struct GestureController {
    /// The template store that is referenced for gesture detection.
    store: Arc<TemplateStore>,
}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new(store: Arc<TemplateStore>) -> Self {
        Self { store }
    }

    /// Attempt to detect gestures in the given collected model.
    pub fn detect_gesture(&self, model: &Model) {
        // TODO: gesture detection logic for model here

        self.store.detect_gesture(model)
    }
}
