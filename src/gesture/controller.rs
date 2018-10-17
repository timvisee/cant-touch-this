use std::{
    fmt::{self, Display},
    io::Result,
    mem,
    sync::{Arc, Mutex},
};

use fragment::{Fragment, FragmentManager};
use store::TemplateStore;
use types::{Model, Template};

/// Gesture controller, for controlling/orchestrating recognition and recording
#[derive(Debug)]
pub struct GestureController {
    /// The template store that is referenced for gesture detection.
    store: Arc<TemplateStore>,

    /// The state.
    state: Mutex<State>,

    /// A list of detected templates.
    detected: Mutex<Vec<Template>>,

    /// The fragment manager.
    pub fragment_manager: Mutex<Option<Arc<FragmentManager>>>,
}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new(store: Arc<TemplateStore>) -> Self {
        Self {
            store,
            state: Mutex::new(State::default()),
            detected: Mutex::new(Vec::new()),
            fragment_manager: Mutex::new(None),
        }
    }

    /// Create a new template based on the current fragment manager data, with the given name and
    /// trim positions.
    pub fn create(&self, name: String, from: usize, to: usize) -> Result<()> {
        // Grab the longest model we can find
        // TODO: improve this later to support multiple fragments in a template
        let mut model = self
            .fragment_manager
            .lock()
            .expect("failed to lock fragment manager to create new template")
            .as_ref()
            .expect("failed to unwrap fragment manager to create new template")
            .longest_model()
            // TODO: do not unwrap, return a proper error
            .expect("no model with trace available");

        // Trim the model
        model.trim(from, to);

        // Create the template
        self.store.add(Template::new(name, model))
    }

    /// Attempt to detect gestures in the given collected fragment.
    #[inline]
    pub fn detect_gesture(&self, fragment: &mut Fragment) {
        // Attempt to find a matching template
        if let Some(template) = self.store.find_matching(fragment) {
            // Clear the history to prevent overlapping detections
            fragment.clear_most();

            // Add the template as detected
            self.add_detected(template);
        }
    }

    /// Get the current gesture controller state.
    pub fn state(&self) -> State {
        *self
            .state
            .lock()
            .expect("failed to lock gesture controller state")
    }

    /// Set the gesture controller state.
    pub fn set_state(&self, state: State) {
        println!("State: {}", state);
        *self
            .state
            .lock()
            .expect("failed to lock gesture controller state") = state;
    }

    /// Add the given template to the list of detected templates.
    /// This function also reports the detected gesture to the console.
    fn add_detected(&self, template: Template) {
        // Report
        println!("# Detected: {}", template.name());

        // TODO: do not clone here
        self
            .detected
            .lock()
            .expect("failed to lock list of detected gestures")
            .push(template);
    }

    /// Flush the list of detected gestures.
    /// The flushed list is returned.
    pub fn flush_detected(&self) -> Vec<Template> {
        // Create a new empty list
        let mut detected = Vec::new();

        // Swap the current list with the empty list
        mem::swap(&mut *self
            .detected
            .lock()
            .expect("failed to lock list of detected gestures"), &mut detected);

        detected
    }

    /// Return live trace data, for visualisation.
    ///
    /// TODO: this is temporary until a better method is implemented.
    pub fn live_trace(&self) -> Vec<Model> {
        match self
            .fragment_manager
            .lock()
            .expect("failed to lock fragment manager")
            .as_ref()
        {
            Some(manager) => manager.live_models(),
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

    /// Clear the current trace data.
    pub fn clear(&self) {
        self.fragment_manager
            .lock()
            .expect("failed to lock fragment manager to clear trace data")
            .as_ref()
            .expect("failed to unwrap fragment manager to clear trace data")
            .clear();
    }
}

/// The state the gesture controller may be in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// The normal and default state.
    Normal,

    /// The recording state.
    Recording,

    /// The saving state, after recording.
    Saving,
}

impl State {
    /// Construct the state from the given ID.
    ///
    /// `None` is returned if the given ID is invalid.
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(State::Normal),
            1 => Some(State::Recording),
            2 => Some(State::Saving),
            _ => None,
        }
    }

    /// Get the state ID.
    pub fn id(&self) -> u8 {
        match self {
            State::Normal => 0,
            State::Recording => 1,
            State::Saving => 2,
        }
    }

    /// Get the state name.
    pub fn name(&self) -> &'static str {
        match self {
            State::Normal => "normal",
            State::Recording => "recording",
            State::Saving => "saving",
        }
    }

    /// Determine whether we should track incomming trace data.
    #[inline]
    pub fn should_track(&self) -> bool {
        match self {
            State::Saving => false,
            _ => true,
        }
    }

    /// Determine whether we should detect gestures in incomming trace data.
    #[inline]
    pub fn should_detect(&self) -> bool {
        match self {
            State::Normal => true,
            _ => false,
        }
    }
}

impl Default for State {
    fn default() -> State {
        State::Normal
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
