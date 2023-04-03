//! Small CLI utility to fetch the official ASTERIX webpage and scrape the Hell of it in order
//! to get the official list of SAC codes.
//!
//! XXX The fact that I even have to do this is an utter failure on the Agency side.

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use log::{debug, info};
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use stderrlog::LogLevelNum::{Debug, Error, Info, Trace};

use crate::cli::Opts;
use crate::parse::{parse_header, parse_tr};
use crate::sac::Area;
use crate::version::version;

pub mod cli;
pub mod parse;
pub mod sac;
pub mod version;

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
    let doc = get(PAGE)?.text()?;
    let today: DateTime<Utc> = Utc::now();

    // We want <table> because sometimes there are 3 <td> and sometimes 2 inside a <tr>.
    //
    let sel = Selector::parse("table").unwrap();

    // Parse the page
    //
    let doc = Html::parse_document(&doc);

    // Load the different tabs' header
    //
    let hdrs = parse_header(&doc)?;

    // Define a regex to sanitize some data, don't ask me why some entries have an embedded
    // <br> or <br />.  Makes no sense to me.
    //
    let re = Regex::new(r##"<br>"##).unwrap();

    // Now look into every table header and table in parallel
    //
    hdrs.iter().zip(doc.select(&sel)).for_each(|(n, e)| {
        // For each line
        //
        debug!("frag={:?}", e.html());

        info!("Table");

        // Now we want each <tr>
        //
        let sel = Selector::parse("tr").unwrap();
        let iter = e.select(&sel);

        let mut area = Area::new(n);

        iter.for_each(|e| {
            debug!("td={e:?}");
            let frag = e.html();

            // Filter
            //
            let frag = re.replace_all(&frag, "");

            // Get what we want
            //
            let (_, (a, b)) = parse_tr(&frag).unwrap();
            if !a.contains("SAC") {
                area.add(a, b);
            }
        });
        println!("area={}\n", area);
    });
    info!("Information retrieved on: {}", today);
    Ok(())
}
