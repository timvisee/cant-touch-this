#![feature(
    decl_macro,
    plugin,
    proc_macro_hygiene,
    option_replace,
    test,
    euclidean_division
)]

#[macro_use]
extern crate clap;
extern crate itertools;
extern crate leap;
extern crate nalgebra;
extern crate openssl_probe;
extern crate rayon;
#[macro_use]
extern crate rocket;
#[cfg(feature = "web")]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate test;
extern crate toml;
extern crate webbrowser;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub(crate) mod config;
pub(crate) mod core;
pub(crate) mod fragment;
pub(crate) mod gesture;
pub(crate) mod prelude;
pub(crate) mod sensor;
pub(crate) mod store;
pub(crate) mod types;
pub(crate) mod util;
#[cfg(feature = "web")]
pub(crate) mod web;

use clap::{App, Arg};
use openssl_probe::init_ssl_cert_env_vars;

use core::Core;

fn main() {
    // Build the clap app, get the matches
    let matches = build_app().get_matches();

    // Initialize SSL certificate variables
    init_ssl_cert_env_vars();

    // Initialize the core, and start it
    let mut core = Core::new(matches);
    core.start().expect("failed to start core");
    core.stop().expect("failed to gracefully stop core");
}

/// Build the `clap` `App` definition for CLI argument parsing.
fn build_app<'a, 'b>() -> App<'a, 'b> {
    // Define the base application
    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());

    // Define the open argument if web is enabled
    #[cfg(feature = "web")]
    {
        app = app.arg(
            Arg::with_name("open")
                .short("o")
                .long("open")
                .help("Open the web configuration in your browser"),
        );
    }

    app
}
