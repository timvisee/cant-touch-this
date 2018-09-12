use leap::vector::Vector;
use nalgebra::base::Vector3;
use nalgebra::geometry::Rotation3;

/// The maximum number of points allowed in a trace.
///
/// TODO: dynamically define this, based on the longest recorded trace template.
pub const TRACE_MAX_POINTS: usize = 100;

#[derive(Clone, Debug, PartialEq)]
pub struct PointTrace {
    /// The trace points.
    points: Vec<Point3>,
}

impl PointTrace {
    /// Construct a point trace with the given points.
    pub fn new(v: Vec<Point3>) -> PointTrace {
        PointTrace { points: v }
    }

    /// Construct an emtpy point trace.
    pub fn empty() -> PointTrace {
        PointTrace { points: vec![] }
    }

    /// Convert this point trace into a rotational trace.
    pub fn to_rot_trace(&self) -> RotTrace {
        let directions: Vec<Rotation3<f64>> = self
            .points
            .windows(2)
            .map(|v| {
                let rotation = Rotation3::rotation_between(
                    &v[0].to_algebra_vector(),
                    &v[1].to_algebra_vector()
                );
                rotation.expect("Failed to determine rotation between vectors")
            }).collect();

        let rot_points: Vec<RotPoint> = directions
            .windows(2)
            .map(|r| {
                RotPoint::new(r[0].angle_to(&r[1]))
            })
            .collect();

        RotTrace::new(rot_points)
    }

    /// Add a new point to the trace.
    pub fn push(&mut self, point: Point3) {
        self.points.push(point);
        self.truncate();
    }

    /// Truncate the trace to the maximum allowed points.
    ///
    /// This removes the oldest points from the trace to fit `TRACE_MAX_POINTS`.
    /// If the maximum isn't reached yet, invoking this does nothing.
    ///
    /// TODO: do not apply this when recording a trace, as it may have any length.
    fn truncate(&mut self) {
        if self.points.len() > TRACE_MAX_POINTS {
            let truncate = self.points.len() - TRACE_MAX_POINTS;
            self.points.drain(..truncate);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotTrace {
    /// The rotation at each point.
    points: Vec<RotPoint>,
}

impl RotTrace {
    pub fn new(v: Vec<RotPoint>) -> RotTrace {
        RotTrace { points: v }
    }

    pub fn empty() -> RotTrace {
        RotTrace { points: vec![] }
    }
}

/// A rotational point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RotPoint(f64);

impl RotPoint {
    pub fn new(f: f64) -> RotPoint {
        RotPoint(f)
    }

    pub fn zero() -> RotPoint {
        RotPoint(0.0)
    }
}

/// A point in 3D space.
// TODO: replace with leap library construct, or a more generic
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3 {
    /// The X coordinate.
    x: f64,

    /// The Y coordinate.
    y: f64,

    /// The Z coordinate.
    z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn zero() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_leap(v: Vector) -> Point3 {
        Point3 {
            x: v.x().into(),
            y: v.y().into(),
            z: v.z().into(),
        }
    }

    pub fn to_algebra_vector(&self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl From<Vector> for Point3 {
    fn from(v: Vector) -> Self {
        Point3::from_leap(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_to_rot_trace() {
        let points = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(5.0, 5.0, 5.0),
        ];
        
        let rotation = vec![RotPoint::new(0.0)];

        let point_trace = PointTrace::new(points);
        let rotation_trace = RotTrace::new(rotation);

        assert_eq!(point_trace.to_rot_trace(), rotation_trace);
    }
}
