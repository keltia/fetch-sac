//! Small CLI utility to fetch the official ASTERIX webpage and scrape the Hell of it in order
//! to get the official list of SAC codes.
//!
//! XXX The fact that I even have to do this is an utter failure on the Agency side.

use std::fs;
use std::time::Instant;

use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use log::{debug, info};
use reqwest::blocking::get;
use stderrlog::LogLevelNum::{Debug, Error, Info, Trace};

use crate::cli::Opts;
use crate::core::{prepare_data, scrape_data, to_csv};
use crate::version::version;

mod cli;
mod core;
mod version;

const PAGE: &str = "https://www.eurocontrol.int/asterix";

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Exit if needed
    //
    if opts.version {
        return Ok(());
    }
    // Check verbosity
    //
    let lvl = match opts.verbose {
        0 => Info,
        1 => Error,
        2 => Debug,
        3 => Trace,
        _ => Trace,
    };

    // Prepare logging.
    stderrlog::new()
        .modules([module_path!()])
        .quiet(opts.quiet)
        .verbosity(lvl)
        .init()?;

    // Add banner
    //
    info!("{}\n", version());

    debug!("Debug mode engaged");

    // Fetch the official page
    //
    let now = Instant::now();
    let doc = get(PAGE)?.text()?;
    let now = now.elapsed().as_millis();

    info!("Fetch took {} ms", now);

    // Time it
    //
    let now = Instant::now();
    let areas = scrape_data(doc)?;
    let now = now.elapsed().as_millis();

    info!("Processing took {} ms", now);

    // get everything into `data` as a String, will be either json, csv or plain text
    //
    let data: String = if opts.json {
        // Info json directly
        //
        serde_json::to_string(&areas)?
    } else if opts.csv {
        // Flatten the different areas into one
        //
        to_csv(prepare_data(&areas)?)?
    } else {
        // Just plain text,  prettier than just `dbg!()`
        //
        areas
            .iter()
            .map(|a| format!("{a}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Write output
    //
    match opts.output {
        Some(output) => {
            info!("Writing {}...", output.to_string_lossy());
            fs::write(output, data)?
        }
        _ => println!("{}", data),
    }

    info!("Information retrieved on: {}", Utc::now());
    Ok(())
}
