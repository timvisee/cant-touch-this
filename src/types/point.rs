use std::fmt;

use leap::vector::Vector;
use nalgebra::{base::Vector3, geometry::Point3 as NPoint3};

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

/// A rotational point.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RotPoint(f64);

impl Point3 {
    /// Create a new Point3 using three `f64`'s.
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
    }

    /// Create a new Point3 with all coordinates set to `0.0`.
    pub fn zero() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create a new Point3 using a `&leap::vector::Vector`.
    pub fn from_leap(v: &Vector) -> Point3 {
        Point3 {
            x: v.x().into(),
            y: v.y().into(),
            z: v.z().into(),
        }
    }

    /// Convert this point to a `nalgebra` `Point3` used for special
    /// calculations.
    pub fn to_npoint(&self) -> NPoint3<f64> {
        NPoint3::new(self.x, self.y, self.z)
    }
}

impl RotPoint {
    pub fn new(f: f64) -> RotPoint {
        RotPoint(f)
    }

    pub fn zero() -> RotPoint {
        RotPoint(0.0)
    }

    pub fn from_degrees(degrees: f64) -> RotPoint {
        RotPoint(degrees.to_radians())
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for RotPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Vector> for Point3 {
    fn from(v: Vector) -> Self {
        Point3::from_leap(&v)
    }
}
