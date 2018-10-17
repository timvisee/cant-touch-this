use std::cmp::max;

use itertools::Itertools;
use nalgebra::geometry;
use std::fmt;

use config::{recognition::KEEP_POINTS, trace::MAX_POINTS};
use prelude::*;
use types::{Point3, RotPoint};
use util::rad::diff as rad_diff;

/// The 2D point type we're using
type NPoint2 = geometry::Point2<f64>;

/// The 3D point type we're using
type NPoint3 = geometry::Point3<f64>;

#[derive(Clone, Debug, PartialEq)]
pub struct PointTrace {
    /// The trace points.
    points: Vec<Point3>,
}

impl PointTrace {
    /// Constructor.
    #[allow(unused)]
    pub fn new(v: Vec<Point3>) -> Self {
        Self { points: v }
    }

    /// Construct an emtpy point trace.
    pub fn empty() -> Self {
        Self { points: vec![] }
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
        Self::calc_rot_points_iter(points.into_iter().map(|p| p.to_npoint())).collect()
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
    fn calc_rot_points_iter<'a, I>(points: I) -> impl Iterator<Item = RotPoint> + 'a
    where
        I: Iterator<Item = NPoint3> + 'a,
    {
        points
            .map(|p| NPoint2::new(p.x, p.y))
            .tuple_windows()
            .map(|(a, b)| b - a)
            .tuple_windows()
            .map(|(a, b)| (rad_diff(b.y.atan2(b.x), a.y.atan2(a.x)), a.magnitude()))
            .map(RotPoint::from_tuple)
    }

    /// Given a list of points, calculate the rotation/angle the edges between
    /// points in radians.
    ///
    /// In order to make reliable calculations the first two points are dropped
    /// in the result. If a list of less than 3 points is given, an emtpy result
    /// is returned.
    #[inline]
    fn to_rot_points(&self, resample: bool) -> Vec<RotPoint> {
        if resample {
            Self::calc_rot_points_iter(self.points.iter().map(|p| p.to_npoint()).sample_points())
                .collect()
        } else {
            Self::calc_rot_points_iter(self.points.iter().map(|p| p.to_npoint())).collect()
        }
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
    #[allow(unused)]
    #[inline]
    pub fn to_last_rot_point(&self) -> Option<RotPoint> {
        Self::calc_rot_points(self.points.split_at(max(self.points.len(), 3) - 3).1)
            .first()
            .cloned()
    }

    /// Convert this point trace into a rotational trace.
    #[allow(unused)]
    #[inline]
    pub fn to_rot_trace(&self, resample: bool) -> RotTrace {
        RotTrace::new(self.to_rot_points(resample))
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

    /// Clear most of the trace, except for the last few (newest) points as
    /// specified in `config::recognition::KEEP_POINTS`.
    ///
    /// The the number of current points is the same or less than
    /// `KEEP_POINTS`, no points are removed from the trace.
    pub fn clear_most(&mut self) {
        let len = self.points.len();
        if len > KEEP_POINTS {
            self.points.drain(..len - KEEP_POINTS);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RotTrace {
    /// The rotation at each point.
    points: Vec<RotPoint>,
}

impl RotTrace {
    /// Constructor.
    pub fn new(v: Vec<RotPoint>) -> Self {
        Self { points: v }
    }

    /// Construct an empty rotational trace.
    pub fn empty() -> RotTrace {
        Self { points: vec![] }
    }

    /// Get a reference to the rotation points in this trace.
    pub fn points(&self) -> &Vec<RotPoint> {
        &self.points
    }

    /// Get the number of points in this trace.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Trim the trace to the given bounds `[from, to]`.
    ///
    /// The bounds may be greater than the actual trace itself.
    pub fn trim(&mut self, from: usize, to: usize) {
        self.points = self
            .points
            .iter()
            .skip(from)
            .take(to - from)
            .cloned()
            .collect();
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

        assert_eq!(zero.to_rot_trace(false), RotTrace::empty());
        assert_eq!(one.to_rot_trace(false), RotTrace::empty());
        assert_eq!(two.to_rot_trace(false), RotTrace::empty());
        assert!(zero.to_last_rot_point().is_none());
        assert!(one.to_last_rot_point().is_none());
        assert!(two.to_last_rot_point().is_none());
    }

    #[test]
    fn straight() {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(5.0, 5.0, 0.0),
        ]);

        let expected = RotTrace::new(vec![RotPoint::new(0f64, 2f64.sqrt())]);

        assert_eq!(points.to_rot_trace(false), expected);

        // TODO: is this obsolete?
        assert_eq!(
            points.to_last_rot_point(),
            Some(RotPoint::new(0f64, 2f64.sqrt()))
        );
    }

    #[test]
    fn corner() {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 5.0, 0.0),
            Point3::new(5.0, 5.0, 0.0),
            Point3::new(5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ]);

        let expected = RotTrace::new(vec![RotPoint::from_degrees(-90.0, 5.0); 3]);

        assert_eq!(points.to_rot_trace(false), expected);
        assert_eq!(
            points.to_last_rot_point(),
            Some(RotPoint::from_degrees(-90.0, 5.0))
        );
    }

    #[bench]
    fn corner_bench(b: &mut Bencher) {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 5.0, 0.0),
            Point3::new(5.0, 5.0, 0.0),
            Point3::new(5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ]);

        b.iter(|| black_box(points.to_rot_trace(false)));
    }

    #[bench]
    fn corner_bench_4096(b: &mut Bencher) {
        let points = PointTrace::new(vec![Point3::zero(); 4096]);

        b.iter(|| black_box(points.to_rot_trace(false)));
    }

    #[bench]
    fn corner_bench_resampled(b: &mut Bencher) {
        let points = PointTrace::new(vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 5.0, 0.0),
            Point3::new(5.0, 5.0, 0.0),
            Point3::new(5.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ]);

        b.iter(|| black_box(points.to_rot_trace(true)));
    }

    #[bench]
    fn corner_bench_4096_resampled(b: &mut Bencher) {
        let points = PointTrace::new(vec![Point3::zero(); 4096]);

        b.iter(|| black_box(points.to_rot_trace(true)));
    }
}
