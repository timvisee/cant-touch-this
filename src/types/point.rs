use std::fmt;

use leap::vector::Vector;
use nalgebra::geometry::Point3 as NPoint3;

/// A point in 3D space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3 {
    /// The X coordinate.
    pub x: f64,

    /// The Y coordinate.
    pub y: f64,

    /// The Z coordinate.
    pub z: f64,
}

impl Point3 {
    /// Create a new Point3 using three `f64`'s.
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
    }

    /// Create a new Point3 using a `&leap::vector::Vector`.
    pub fn from_leap(v: &Vector) -> Point3 {
        Self::new(v.x().into(), v.y().into(), v.z().into())
    }

    /// Convert this point to a `nalgebra` `Point3` used for special
    /// calculations.
    pub fn to_npoint(&self) -> NPoint3<f64> {
        NPoint3::new(self.x, self.y, self.z)
    }
}

/// A rotational point.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RotPoint {
    /// The angle for this rotation.
    angle: f64,

    /// The distance to the next point.
    distance: f64,
}

impl RotPoint {
    /// Construct a new rotational point.
    pub fn new(angle: f64, distance: f64) -> Self {
        Self { angle, distance }
    }

    /// Construct a new rotational point with it's components in a tuple.
    /// This may be used for iterators, like: `.map(RotPoint::from_tuple)`
    pub fn from_tuple((angle, distance): (f64, f64)) -> Self {
        Self::new(angle, distance)
    }

    /// Create a rotational point based on the given number of degrees instead of using radians.
    #[allow(unused)]
    pub fn from_degrees(degrees: f64, distance: f64) -> Self {
        Self::new(degrees.to_radians(), distance)
    }

    /// Get the number of radians for this rotational point.
    pub fn radians(&self) -> f64 {
        self.angle
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for RotPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} rad -> {})", self.angle, self.distance)
    }
}

impl From<Vector> for Point3 {
    fn from(v: Vector) -> Self {
        Point3::from_leap(&v)
    }
}
