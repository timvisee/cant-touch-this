//! Types used throughout the crate

pub mod model;
pub mod template;
pub mod trace;

/// Re-exports
pub use self::model::Model;
pub use self::template::Template;
pub use self::trace::{Point3, PointTrace, RotPoint, RotTrace};
