use std::{
    fmt::{self, Display},
    sync::{Arc, Mutex},
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

    /// The state.
    state: Mutex<State>,

    /// The fragment manager.
    /// TODO: this is temporary, and should not be public
    pub fragment_manager: Mutex<Option<Arc<FragmentManager>>>,
}

impl GestureController {
    /// Construct a new gesture controller.
    pub fn new(store: Arc<TemplateStore>) -> Self {
        Self {
            store,
            state: Mutex::new(State::default()),
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
    pub fn should_track(&self) -> bool {
        match self {
            State::Saving => false,
            _ => true,
        }
    }

    /// Determine whether we should detect gestures in incomming trace data.
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
