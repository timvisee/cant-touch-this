pub struct PointTrace {
    /// The trace points.
    points: Vec<Point3>,
}

pub struct RotTrace {
    /// The rotation at each point.
    points: Vec<RotPoint>,
}

/// A rotational point.
pub struct RotPoint(f64);

/// A point in 3D space.
// TODO: replace with leap library construct, or a more generic
pub struct Point3 {
    /// The X coordinate.
    x: f64,

    /// The Y coordinate.
    y: f64,

    /// The Z coordinate.
    z: f64,
}

// TODO: Point3::new()
// TODO: Point3::from_leap()
