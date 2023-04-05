//! Core module dealing with the data we operate on
//!
//! The main scraper is defined here, calling parts of the module like the parser to prepare data, etc.
//!

use anyhow::Result;
use log::{debug, info};
use regex::Regex;
use scraper::{Html, Selector};

// Re-export for shorter paths
//
pub use area::*;
pub use csv_output::*;
pub use parse::*;
pub use sac::*;

pub mod area;
pub mod csv_output;
pub mod parse;
pub mod sac;

/// Extract the HTML stuff from the page
///
pub fn scrape_data(doc: String) -> Result<Vec<Area>> {
    // Define a regex to sanitize some data, don't ask me why some entries have an embedded
    // <br> or <br />.  Makes no sense to me.
    //
    let re = Regex::new(r##"<br>"##).unwrap();

    // Parse the page
    //
    let doc = Html::parse_document(&doc);

    // Load the different tabs' header
    //
    let hdrs = parse_header(&doc)?;

    info!("{} regions found", hdrs.len());

    // We want <table> because sometimes there are 3 <td> and sometimes 2 inside a <tr>.
    //
    let sel = Selector::parse("table").unwrap();

    // Now look into every table header and table in parallel
    //
    Ok(hdrs
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
        .collect::<Vec<_>>())
}
