use itertools::Itertools;
use nalgebra::geometry;
use std::{
    cmp::max,
    f64::consts::PI,
    fmt,
};

use config::trace::MAX_POINTS;
use prelude::*;
use types::{Point3, RotPoint};

/// The 2D point type we're using
type NPoint2 = geometry::Point2<f64>;

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
    fn calc_rot_points(points: &[Point3]) -> Vec<RotPoint> {
        Self::calc_rot_points_iter(points).collect()
    }

    /// Given a list of points, calculate the rotation/angle the edges between
    /// points in radians.
    ///
    /// In order to make reliable calculations the first two points are dropped
    /// in the result. If a list of less than 3 points is given, an emtpy result
    /// is returned.
    ///
    /// A streaming/lazy iterator is returned for optimal performance.
    ///
    /// TODO: dynamically determine drawing plane at `NPoint2` conversion point
    #[inline]
    fn calc_rot_points_iter<'a>(points: &'a [Point3]) -> impl Iterator<Item = RotPoint> + 'a {
        // Resample the points, then do the rotatoinal calculations
        points
            .iter()
            .map(|p| p.to_npoint())
            .sample_points()
            .map(|p| NPoint2::new(p.x, p.y))
            .tuple_windows()
            .map(|(a, b)| b - a)
            .tuple_windows()
            .map(|(a, b)| (b.y.atan2(b.x) - a.y.atan2(a.x), a.magnitude()))
            .map(RotPoint::from_tuple)
    }

    /// Given a list of points, calculate the rotation/angle the edges between
    /// points in radians.
    ///
    /// In order to make reliable calculations the first two points are dropped
    /// in the result. If a list of less than 3 points is given, an emtpy result
    /// is returned.
    fn to_rot_points(&self) -> Vec<RotPoint> {
        Self::calc_rot_points(&self.points)
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
        Self::calc_rot_points(self.points.split_at(max(self.points.len(), 3) - 3).1)
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
    /// This removes the oldest points from the trace to fit `config::trace::MAX_POINTS`.
    /// If the maximum isn't reached yet, invoking this does nothing.
    ///
    /// TODO: do not apply this when recording a trace, as it may have any
    /// length.
    #[inline]
    fn truncate(&mut self) {
        if self.points.len() > MAX_POINTS {
            let truncate = self.points.len() - MAX_POINTS;
            self.points.drain(..truncate);
        }
    }

    /// Clear the trace.
    ///
    /// This resets the trace back to zero items.
    pub fn clear(&mut self) {
        self.points.clear();
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
    /// This removes the oldest points from the trace to fit `MAX_POINTS`.
    /// If the maximum isn't reached yet, invoking this does nothing.
    ///
    /// TODO: do not apply this when recording a trace, as it may have any
    /// length.
    #[inline]
    fn truncate(&mut self) {
        if self.points.len() > MAX_POINTS {
            let truncate = self.points.len() - MAX_POINTS;
            self.points.drain(..truncate);
        }
    }

    /// Clear the trace.
    ///
    /// This resets the trace back to zero items.
    pub fn clear(&mut self) {
        self.points.clear();
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
    use test::{black_box, Bencher};

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

    #[bench]
    fn corner_bench(b: &mut Bencher) {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 5.0, 0.0),
            Point3::new(0.0, 5.0, 5.0),
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(-5.0, 0.0, 5.0),
            Point3::new(-5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ]);

        b.iter(|| black_box(points.to_rot_trace()));
    }
}
