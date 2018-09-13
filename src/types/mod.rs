//! Types used throughout the crate

pub mod model;
pub mod point;
pub mod template;
pub mod trace;

/// Re-exports
pub use self::model::Model;
pub use self::{
    point::{Point3, RotPoint},
    trace::{PointTrace, RotTrace},
};
pub use self::template::Template;
