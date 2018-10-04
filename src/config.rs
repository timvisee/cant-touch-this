//! A module holding some centralised configurable properties.

/// Sampling related configuration.
pub mod sample {
    /// The distance that is used between points when resampling a trace.
    pub const DISTANCE: f64 = 15.0;
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
    /// The maximum allowed angle difference threshold in radians of the cumulative angles
    /// between a trace and a trace template.
    pub const TOTAL_DIFF_MAX: f64 = 0.09;

    /// The maximum angle difference threshold in radians any trace point may have as compared to
    /// a template.
    pub const POINT_DIFF_MAX: f64 = 2.0;

    /// The size of comparison groups.
    ///
    /// Overlapping groups of points are selected to be compared, with averaged out values.
    /// This defines how big these comparison groups are.
    pub const GROUP_SIZE: usize = 5;

    /// The maximum average angle difference threshold in radians a group of points may have as
    /// compared to a template.
    pub const GROUP_DIFF_MAX: f64 = 1.5;
}

/// Template related configuratoin.
pub mod template {
    /// The file path to save the templates in.
    pub const FILE: &str = "~/.config/cant-touch-this/templates.toml";
}
