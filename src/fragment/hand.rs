use std::collections::HashMap;

use leap::FingerType;

use super::Fragment;

/// A hand with traces.
pub struct Hand {
    /// The fingers on this hand, grouped by their finger types.
    fingers: HashMap<FingerType, Fragment>,
}

impl Hand {
    /// Construct a new hand.
    pub fn new() -> Self {
        Hand {
            fingers: HashMap::new(),
        }
    }
}
