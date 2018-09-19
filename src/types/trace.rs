use std::fmt;

use nalgebra::geometry::{Point3 as NPoint3, Rotation3};

use types::{Point3, RotPoint};

/// The maximum number of points allowed in a trace.
///
/// TODO: dynamically define this, based on the longest recorded trace template.
pub const TRACE_MAX_POINTS: usize = 100;

#[derive(Clone, Debug, PartialEq)]
pub struct PointTrace {
    /// The trace points.
    points: Vec<Point3>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotTrace {
    /// The rotation at each point.
    points: Vec<RotPoint>,
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

    /// Given a list of points, calculate the rotation/angle the edges between
    /// points in radians.
    ///
    /// In order to make reliable calculations the first two points are dropped
    /// in the result. If a list of less than 3 points is given, an emtpy result
    /// is returned.
    pub fn calc_point_angles(&self) -> RotTrace {
        let rot_points = self
            .points
            .iter()
            .map(|p| NPoint3::new(p.x, p.y, p.z))
            .collect::<Vec<_>>()
            .windows(2)
            .map(|p| p[1] - p[0])
            .collect::<Vec<_>>()
            .windows(2)
            .map(|p| p[0].angle(&p[1]))
            .map(|p| RotPoint::new(p))
            .collect();

        RotTrace::new(rot_points)
    }

    /// Convert this point trace into a rotational trace.
    pub fn to_rot_trace(&self) -> RotTrace {
        let directions: Vec<Rotation3<f64>> = self
            .points
            .windows(2)
            .map(|v| {
                let rotation = Rotation3::rotation_between(
                    &v[0].to_algebra_vector(),
                    &v[1].to_algebra_vector(),
                );
                rotation.expect("Failed to determine rotation between vectors")
            }).collect();

        let rot_points: Vec<RotPoint> = directions
            .windows(2)
            .map(|r| RotPoint::new(r[0].angle_to(&r[1])))
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

impl RotTrace {
    pub fn new(v: Vec<RotPoint>) -> RotTrace {
        RotTrace { points: v }
    }

    pub fn empty() -> RotTrace {
        RotTrace { points: vec![] }
    }
}

impl fmt::Display for PointTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = self
            .points
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}", p)
    }
}

impl fmt::Display for RotTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = self
            .points
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}", p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        let zero = PointTrace::empty();
        let one = PointTrace::new(vec![Point3::zero(); 1]);
        let two = PointTrace::new(vec![Point3::zero(); 2]);

        assert_eq!(zero.calc_point_angles(), RotTrace::empty());
        assert_eq!(one.calc_point_angles(), RotTrace::empty());
        assert_eq!(two.calc_point_angles(), RotTrace::empty());
    }

    #[test]
    fn straight() {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(5.0, 5.0, 5.0),
        ]);

        let expected = RotTrace::new(vec![RotPoint::zero(); 1]);

        assert_eq!(points.calc_point_angles(), expected);
    }

    #[test]
    fn corner() {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 5.0, 0.0),
            Point3::new(0.0, 5.0, 5.0),
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(-5.0, 0.0, 5.0),
            Point3::new(-5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ]);

        let expected = RotTrace::new(vec![RotPoint::from_degrees(90.0); 5]);

        assert_eq!(points.calc_point_angles(), expected)
    }
}
