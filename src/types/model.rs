use std::cmp::min;

use config::recognition::{GROUP_DIFF_MAX, GROUP_SIZE, POINT_DIFF_MAX, TOTAL_DIFF_MAX};
use types::RotTrace;
use util::rad::diff as rad_diff;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    /// Get a reference to the internal rotational trace.
    pub fn trace(&self) -> &RotTrace {
        &self.trace
    }

    /// Get a mutable reference to the internal rotational trace.
    pub fn trace_mut(&mut self) -> &mut RotTrace {
        &mut self.trace
    }

    /// Clear the model.
    ///
    /// This resets the model back to zero trace items.
    pub fn clear(&mut self) {
        self.trace.clear();
    }

    /// Compare this model against the given `other` model, and decide whether they are similar
    /// enough to match.
    ///
    /// This is used for gesture detection.
    pub fn matches(&self, other: &Model) -> bool {
        // Get the model and other model points
        let model_points = self.trace.points();
        let other_points = other.trace.points();
        let model_count = model_points.len();
        let other_count = other_points.len();

        // Skip if the template has more points than our current trace
        if other_count < model_count {
            return false;
        }

        // Determine how many points to process, minimum length wins
        let count = min(model_points.len(), other_points.len());

        // Select the last points based on the determined count to use
        let model_points = &model_points[model_count - count..model_count];
        let other_points = &other_points[other_count - count..other_count];

        // Caluclate the difference for each point
        let diff = model_points
            .iter()
            .rev()
            .zip(other_points.iter().rev())
            .map(|(a, b)| rad_diff(b.radians(), a.radians()));

        // Calculate the cummulative difference on each point
        let cum_diff: Vec<f64> = diff
            .scan(0.0, |acc, p| {
                *acc += p;
                Some(*acc)
            })
            .collect();

        // Skip if the total difference is too big
        if cum_diff.last().unwrap().abs() > TOTAL_DIFF_MAX {
            return false;
        }

        // Skip if any of the points has a difference of more than 2
        if cum_diff.iter().any(|p| p.abs() > POINT_DIFF_MAX) {
            return false;
        }

        // Skip if each window of 5 points has an average difference bigger than 1
        if GROUP_SIZE > 0 && cum_diff
            .windows(GROUP_SIZE)
            .any(|p| (p.iter().sum::<f64>().abs() / GROUP_SIZE as f64) > GROUP_DIFF_MAX)
        {
            return false;
        }

        true
    }
}
