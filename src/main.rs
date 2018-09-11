extern crate leap;

pub(crate) mod beautifier;
pub(crate) mod core;
pub(crate) mod gesture;
pub(crate) mod sensor;
pub(crate) mod store;
pub(crate) mod types;

use core::Core;

fn main() {
    println!("Can't touch this");

    // Initialize the core
    let core = Core::new();

    println!("Done");
}
