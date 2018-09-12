#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate leap;
extern crate rocket;
extern crate rocket_contrib;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub(crate) mod beautifier;
pub(crate) mod core;
pub(crate) mod gesture;
pub(crate) mod sensor;
pub(crate) mod store;
pub(crate) mod types;
pub(crate) mod web;

use core::Core;

fn main() {
    println!("Can't touch this");

    // Initialize the core
    let core = Core::new();

    println!("Done");
}
