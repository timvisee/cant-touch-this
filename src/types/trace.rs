use std::cmp::max;
use std::f64::consts::PI;
use std::fmt;

use itertools::Itertools;
use nalgebra::geometry::Point2 as NPoint2;
use nalgebra::geometry::Point3 as NPoint3;

use types::{Point3, RotPoint};

/// The maximum number of points allowed in a trace.
///
/// TODO: dynamically define this, based on the longest recorded trace template.
pub const TRACE_MAX_POINTS: usize = 1024;

#[derive(Clone, Debug, PartialEq)]
pub struct PointTrace {
    /// The trace points.
    points: Vec<Point3>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    ///
    /// TODO: stream the iterator result, don't collect, improve performance
    #[inline]
    fn calc_rot_points(&self, points: &[Point3]) -> Vec<RotPoint> {
        // TODO: define a plane to multiply each point with
        // TODO: dynamically determine this plane

        points
            .iter()
            // TODO: use a 3D point, we're in 3D space
            // .map(|p| p.to_npoint())
            .map(|p| NPoint2::new(p.x, p.y))
            .tuple_windows()
            .map(|(a, b)| b - a)
            .tuple_windows()
            .map(|(a, b)| (b.y.atan2(b.x) - a.y.atan2(a.x), a.magnitude()))
            .map(RotPoint::from_tuple)
            .collect()
    }

    /// Given a list of points, calculate the rotation/angle the edges between
    /// points in radians.
    ///
    /// In order to make reliable calculations the first two points are dropped
    /// in the result. If a list of less than 3 points is given, an emtpy result
    /// is returned.
    fn to_rot_points(&self) -> Vec<RotPoint> {
        self.calc_rot_points(&self.points)
    }

    /// Given a list of points (wrapped by this trace), calculate the last
    /// rotation/angle between the last two edges of the points, in radians.
    ///
    /// This function is similar to `calc_rot_points`, but only calculates the
    /// last rotational point instead of all known. This may be used for
    /// incremental rotation trace creation.
    ///
    /// At least three points need to be in this list in order to return the
    /// last rotation. If that isn't the case, `None` is returned instead.
    pub fn to_last_rot_point(&self) -> Option<RotPoint> {
        self.calc_rot_points(self.points.split_at(max(self.points.len(), 3) - 3).1)
            .first()
            .cloned()
    }

    /// Convert this point trace into a rotational trace.
    #[allow(unused)]
    pub fn to_rot_trace(&self) -> RotTrace {
        RotTrace::new(self.to_rot_points())
    }

    /// Add a new point to the trace.
    #[inline]
    pub fn push(&mut self, point: Point3) {
        self.points.push(point);
        self.truncate();
    }

    /// Truncate the trace to the maximum allowed points.
    ///
    /// This removes the oldest points from the trace to fit `TRACE_MAX_POINTS`.
    /// If the maximum isn't reached yet, invoking this does nothing.
    ///
    /// TODO: do not apply this when recording a trace, as it may have any
    /// length.
    #[inline]
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

    /// Push the given rotational point on the trace.
    pub fn push(&mut self, point: RotPoint) {
        self.points.push(point);
        self.truncate();
    }

    /// Get a reference to the rotation points in this trace.
    pub fn points(&self) -> &Vec<RotPoint> {
        &self.points
    }

    /// Truncate the trace to the maximum allowed points.
    ///
    /// This removes the oldest points from the trace to fit `TRACE_MAX_POINTS`.
    /// If the maximum isn't reached yet, invoking this does nothing.
    ///
    /// TODO: do not apply this when recording a trace, as it may have any
    /// length.
    #[inline]
    fn truncate(&mut self) {
        if self.points.len() > TRACE_MAX_POINTS {
            let truncate = self.points.len() - TRACE_MAX_POINTS;
            self.points.drain(..truncate);
        }
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

        assert_eq!(zero.to_rot_trace(), RotTrace::empty());
        assert_eq!(one.to_rot_trace(), RotTrace::empty());
        assert_eq!(two.to_rot_trace(), RotTrace::empty());
        assert!(zero.to_last_rot_point().is_none());
        assert!(one.to_last_rot_point().is_none());
        assert!(two.to_last_rot_point().is_none());
    }

    #[test]
    fn straight() {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(5.0, 5.0, 5.0),
        ]);

        let expected = RotTrace::new(vec![RotPoint::new(0f64, 3f64.sqrt())]);

        assert_eq!(points.to_rot_trace(), expected);
        assert_eq!(
            points.to_last_rot_point(),
            Some(RotPoint::new(0f64, 3f64.sqrt()))
        );
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

        let expected = RotTrace::new(vec![RotPoint::from_degrees(90.0, 5.0); 5]);

        assert_eq!(points.to_rot_trace(), expected);
        assert_eq!(
            points.to_last_rot_point(),
            Some(RotPoint::from_degrees(90.0, 5.0))
        );
    }
}
