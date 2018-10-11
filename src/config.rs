//! A module holding some centralised configurable properties.

/// Sampling related configuration.
pub mod sample {
    /// The distance that is used between points when resampling a trace.
    pub const DISTANCE: f64 = 10.0;
}

/// Trace related configuration.
pub mod trace {
    /// The maximum number of points allowed in a point.
    ///
    /// If traces get longer than this maximum, they will be trimmed automatically.
    pub const MAX_POINTS: usize = 2048;
}

/// Recognition related configuration.
pub mod recognition {
    /// The maximum margin two points must be within (angle difference) for them to be considered
    /// a match.
    ///
    /// When comparing two traces with rotations, points of both traces are compared with each
    /// other. The difference between points must be within the specified margin for them to be
    /// valid.
    pub const MARGIN: f64 = 0.2;

    /// The margin start interrupting comparison searches at.
    ///
    /// When comparing two traces with rotations, for each point in a trace multiple points in the
    /// other trace are tested for a match. If points in the other trace start differing too much
    /// as specified by this value, the search for matching points is stalled and is considered
    /// erroneous.
    pub const INTERRUPT_MARGIN: f64 = 2.75;

    /// The maximum number (inclusive) of allowed match errors when comparing two traces with
    /// rotational points. This defines the maximum number of points allowed that no matching point
    /// could be found for. If this number is exceeded the comparison is aborted.
    pub const MAX_ERROR: usize = 2;

    /// The number of points to consider when searching for a matching point.
    /// When comparing two rotational traces, for each point in the first trace, the specified
    /// number of points in the other trace are considered to search for a match from an internally
    /// tracked search position on the other trace.
    pub const SEARCH_SPACE: usize = 10;

    /// The maximum allowed factor by which the size/speed of two traces with rotational points deviates.
    /// This ensures that when searching for matching points when comparing two rotational traces
    /// the search position for the other trace advances in a proper tempo, and isn't stalled or
    /// advancing too quickly.
    ///
    /// How bigger this value is the easier traces will match that are proportionally much smaller
    /// or bigger.
    /// The value should always be `1` or above.
    ///
    /// If this value is set to `2`, the other trace mad advance half as fast at a minimum or twice
    /// as quickly at a maximum relative to the base trace.
    pub const MAX_DEVIATION_FACTOR: f64 = 1.75;

    /// When a trace is recognized as a gesture, the current trace is mostly cleared to prevent
    /// duplicate detections over the same trace.
    ///
    /// This specifies how many points in the current trace history to keep when clearing the
    /// history because of a recognized gesture.
    pub const KEEP_POINTS: usize = 2;

    // TODO: used for the old recognition logic, consider what to keep
    // /// The maximum allowed angle difference threshold in radians of the cumulative angles
    // /// between a trace and a trace template.
    // pub const TOTAL_DIFF_MAX: f64 = 0.09;

    // /// The maximum angle difference threshold in radians any trace point may have as compared to
    // /// a template.
    // pub const POINT_DIFF_MAX: f64 = 2.5;

    // /// The size of comparison groups.
    // ///
    // /// Overlapping groups of points are selected to be compared, with averaged out values.
    // /// This defines how big these comparison groups are.
    // pub const GROUP_SIZE: usize = 4;

    // /// The maximum average angle difference threshold in radians a group of points may have as
    // /// compared to a template.
    // pub const GROUP_DIFF_MAX: f64 = 1.5;
}

/// Template related configuration.
pub mod template {
    /// The name of the templates file.
    pub const TEMPLATES_FILE: &str = "templates.json";
}
