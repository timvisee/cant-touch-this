#![feature(decl_macro, plugin, proc_macro_non_items)]
#![plugin(rocket_codegen)]
// TODO: remove this after developing
#![allow(unused)]

#[macro_use]
extern crate clap;
extern crate leap;
extern crate nalgebra;
#[macro_use]
extern crate rocket;
#[cfg(feature = "web")]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate webbrowser;

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

use clap::{App, Arg};

use core::Core;

fn main() {
    // Build the clap app, get the matches
    let matches = build_app().get_matches();

    // Initialize the core, and start it
    let mut core = Core::new(matches);
    core.start();
    core.stop();
}

/// Build the `clap` `App` definition for CLI argument parsing.
fn build_app<'a, 'b>() -> App<'a, 'b> {
    // Define the base application
    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());

    // Define the open argument if web is enabled
    #[cfg(feature = "web")] {
        app = app.arg(Arg::with_name("open")
            .short("o")
            .long("open")
            .help("Open the web configuration in your browser"));
    }

    app
}
