#![feature(plugin)]
#![plugin(rocket_codegen)]
// TODO: remove this after developing
#![allow(unused)]

extern crate leap;
extern crate nalgebra;
extern crate rocket;
#[cfg(feature = "web")]
extern crate rocket_contrib;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub(crate) mod beautifier;
pub(crate) mod core;
pub(crate) mod fragment;
pub(crate) mod gesture;
pub(crate) mod sensor;
pub(crate) mod store;
pub(crate) mod types;
#[cfg(feature = "web")]
pub(crate) mod web;

use core::CoreHandle;

fn main() {
    println!("Can't touch this");

    // Initialize the core, create a handle
    let core = CoreHandle::new();

    // Start the web server
    #[cfg(feature = "web")]
    core.core().start_server();

    println!("Done");
}
