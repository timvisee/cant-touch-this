use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use fragment::{Fragment, FragmentManager};
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
    recording: AtomicBool,

    /// The fragment manager.
    /// TODO: this is temporary, and should not be public
    pub fragment_manager: Mutex<Option<Arc<FragmentManager>>>,
}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new(store: Arc<TemplateStore>) -> Self {
        Self {
            store,
            recording: AtomicBool::new(false),
            fragment_manager: Mutex::new(None),
        }
    }

    /// Attempt to detect gestures in the given collected fragment.
    #[inline]
    pub fn detect_gesture(&self, fragment: &mut Fragment) {
        // Attempt to find a matching template
        if let Some(template) = self.store.find_matching(fragment) {
            // Clear the history to prevent overlapping detections
            fragment.clear_most();

            // Report
            println!("### HIT: {}", template.name());
        }
    }

    /// Check whether we're recording.
    pub fn recording(&self) -> bool {
        self.recording.load(Ordering::Relaxed)
    }

    /// Set the recording state.
    pub fn set_recording(&self, recording: bool) {
        // Set the state
        self.recording.store(recording, Ordering::Relaxed);

        // Report
        if recording {
            println!("Started recording");
        } else {
            println!("Stopped recording");
        }
    }

    /// Return live trace data, for visualisation.
    ///
    /// TODO: this is temporary until a better method is implemented.
    pub fn get_live_trace(&self) -> Vec<Model> {
        match self
            .fragment_manager
            .lock()
            .expect("failed to lock fragment manager")
            .as_ref()
        {
            Some(manager) => manager.get_live_models(),
            None => Vec::new(),
        }
    }

    /// Set the fragment manager instance that is used.
    ///
    /// TODO: this is temporary
    pub fn set_fragment_manager(&self, fragment_manager: Arc<FragmentManager>) {
        self.fragment_manager
            .lock()
            .expect("failed to set fragment manager, unable to lock handle mutex")
            .replace(fragment_manager);
    }
}
