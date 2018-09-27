use std::sync::{Arc, Mutex};

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

    /// The recording state.
    recording: Mutex<bool>,
}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new(store: Arc<TemplateStore>) -> Self {
        Self {
            store,
            recording: Mutex::new(false),
        }
    }

    /// Attempt to detect gestures in the given collected model.
    pub fn detect_gesture(&self, model: &Model) {
        // TODO: gesture detection logic for model here

        self.store.detect_gesture(model)
    }

    /// Check whether we're recording.
    pub fn recording(&self) -> bool {
        *self
            .recording
            .lock()
            .expect("failed to lock recording state")
    }

    /// Set the recording state.
    pub fn set_recording(&self, recording: bool) {
        // Set the state
        *self
            .recording
            .lock()
            .expect("failed to lock recording state") = recording;

        // Report
        if recording {
            println!("Started recording");
        } else {
            println!("Stopped recording");
        }
    }
}
