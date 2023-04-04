//! Small CLI utility to fetch the official ASTERIX webpage and scrape the Hell of it in order
//! to get the official list of SAC codes.
//!
//! XXX The fact that I even have to do this is an utter failure on the Agency side.

use std::fs;
use std::time::Instant;

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use log::{debug, info};
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use stderrlog::LogLevelNum::{Debug, Error, Info, Trace};

use crate::cli::Opts;
use crate::core::{parse_header, parse_tr, prepare_data, to_csv, Area};
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
    let today: DateTime<Utc> = Utc::now();

    info!("Fetch took {} ms", now);

    // We want <table> because sometimes there are 3 <td> and sometimes 2 inside a <tr>.
    //
    let sel = Selector::parse("table").unwrap();

    // Parse the page
    //
    let doc = Html::parse_document(&doc);

    // Load the different tabs' header
    //
    let hdrs = parse_header(&doc)?;

    info!("{} regions found", hdrs.len());

    // Define a regex to sanitize some data, don't ask me why some entries have an embedded
    // <br> or <br />.  Makes no sense to me.
    //
    let re = Regex::new(r##"<br>"##).unwrap();

    // Time it
    let now = Instant::now();

    // Now look into every table header and table in parallel
    //
    let areas: Vec<Area> = hdrs
        .iter()
        .zip(doc.select(&sel))
        .map(|(name, e)| {
            // For each line
            //
            info!("Table({})", name);

            debug!("frag={}", e.html());

            // Now we want each <tr>
            //
            let sel = Selector::parse("tr").unwrap();
            let iter = e.select(&sel);

            let mut area = Area::new(name);

            iter.filter(|e| !e.html().contains("SAC")).for_each(|e| {
                debug!("td={e:?}");
                let frag = e.html();

                // Filter
                //
                let frag = re.replace_all(&frag, "");

                // Get what we want
                //
                let (_, (a, b)) = parse_tr(&frag).unwrap();
                area.add(a, b);
            });
            area
        })
        .collect();

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

    info!("Information retrieved on: {}", today);
    Ok(())
}
