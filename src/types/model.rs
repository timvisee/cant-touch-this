use std::cmp::{max, min};

use config::recognition::{
    INTERRUPT_MARGIN, MARGIN, MAX_DEVIATION_FACTOR, MAX_ERROR, SEARCH_SPACE,
};
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
    /// The current model must be part of the `other` given model as a whole.
    /// Thus it is alright if the given `other` model is longer than this model.
    /// It probably won't match if the given `other` model is much shorter.
    ///
    /// This is used for gesture detection.
    #[inline]
    pub fn matches(&self, other: &Model) -> bool {
        // Get iterators over the points, from the end
        let model = self.trace.points().iter().rev();
        let other = other.trace.points().iter().rev();

        // Cumulative rotations in iterators
        let model = model.scan(0.0, |acc, p| {
            *acc += p.radians();
            Some(*acc)
        });
        let other = other.scan(0.0, |acc, p| {
            *acc += p.radians();
            Some(*acc)
        });

        // Collect all other points for now
        let other: Vec<f64> = other.collect();

        // The current search position and error count
        let mut pos = 0;
        let mut err = 0;

        // Loop through all model points
        for (p_pos, p) in model.enumerate() {
            // Make sure the search position doesn't advance too quickly or slowly
            // It should be in bound relative to the current template point index,
            // and must have a max deviation as specified in `MAX_DEVIATION_FACTOR`
            pos = max(
                min(pos, (p_pos as f64 * MAX_DEVIATION_FACTOR) as usize),
                (p_pos as f64 / MAX_DEVIATION_FACTOR) as usize,
            );

            // Find the relative index of the next point close enough to the current template point
            // from `pos` in the `other` iterator with a search space specified in `SEARCH_SPACE`.
            // The search will be cancelled if points differ too much as specified in `INTERRUPT_MARGIN`.
            // If other ends or no valid point is found, `None` will be produced.
            let offset = other
                .iter()
                .skip(pos)
                .take(SEARCH_SPACE)
                .map(|o| (o - p).abs())
                // Interrupt search when maximum margin is exceeded
                .take_while(|o| o <= &INTERRUPT_MARGIN)
                // Find point similar enough to template
                .position(|o| o <= MARGIN);

            // Handle the result that was found, increase the search position by offset
            match offset {
                Some(offset) => pos += offset,
                None => {
                    err += 1;
                    if err >= MAX_ERROR {
                        return false;
                    }
                }
            }
        }

        true

        // TODO: old matching logic, consider what to keep
        // // Get the model and other model points
        // let model_points = self.trace.points();
        // let other_points = other.trace.points();
        // let model_count = model_points.len();
        // let other_count = other_points.len();

        // // Skip if the template has more points than our current trace
        // if other_count < model_count {
        //     return false;
        // }

        // // Determine how many points to process, minimum length wins
        // let count = min(model_points.len(), other_points.len());

        // // Select the last points based on the determined count to use
        // let model_points = &model_points[model_count - count..model_count];
        // let other_points = &other_points[other_count - count..other_count];

        // let model_points_cum: Vec<f64> = model_points
        //     .iter()
        //     .scan(0.0, |acc, p| {
        //         *acc += p.radians();
        //         Some(*acc)
        //     })
        //     .collect();
        // let other_points_cum: Vec<f64> = other_points
        //     .iter()
        //     .scan(0.0, |acc, p| {
        //         *acc += p.radians();
        //         Some(*acc)
        //     })
        //     .collect();

        // // Calculate the difference for each point
        // let diff = model_points
        //     .iter()
        //     .rev()
        //     .zip(other_points.iter().rev())
        //     .map(|(a, b)| rad_diff(b.radians(), a.radians()));

        // // Calculate the cumulative difference for each point
        // let diff_inc_abs_diff: Vec<f64> = model_points_cum
        //     .iter()
        //     .rev()
        //     .zip(other_points_cum.iter().rev())
        //     .map(|(a, b)| (a - b).abs())
        //     .collect();

        // // Calculate the cumulative difference on each point
        // let cum_diff: Vec<f64> = diff
        //     .scan(0.0, |acc, p| {
        //         *acc += p;
        //         Some(*acc)
        //     })
        //     .collect();

        // // Skip if the total difference is too big
        // if cum_diff.last().unwrap().abs() > TOTAL_DIFF_MAX {
        //     return false;
        // }

        // if diff_inc_abs_diff
        //     .windows(8)
        //     .any(|p| p.iter().filter(|p| *p > &0.3).count() > 6)
        // {
        //     return false;
        // }

        // // Skip if any of the points has a difference of more than 2
        // if cum_diff.iter().any(|p| p.abs() > POINT_DIFF_MAX) {
        //     return false;
        // }

        // // Skip if each window of 5 points has an average difference bigger than 1
        // if GROUP_SIZE > 0 && cum_diff
        //     .windows(GROUP_SIZE)
        //     .any(|p| (p.iter().sum::<f64>().abs() / GROUP_SIZE as f64) > GROUP_DIFF_MAX)
        // {
        //     return false;
        // }
    }
}
