//! Module dealing data preparation for CSV output
//!

use anyhow::Result;
use csv::{QuoteStyle, WriterBuilder};
use log::{debug, trace};
use serde::Serialize;
use std::collections::VecDeque;

use crate::core::Area;

/// Take the array with all areas and generate a vector of lines with
/// 1. header names
/// 2. each line with area name in the first field
///
/// We need to flatten the whole structure for `WriterBuilder(`) to function properly, need a
/// single `Iterator` over the data.
///
pub fn prepare_data(areas: &[Area]) -> Result<VecDeque<(String, String, String)>> {
    // Generate our values for the first field
    //
    // Merge the two datasets with the region name as first column aka JOIN.
    //
    trace!("Generate data for csv");
    let mut flat: VecDeque<_> = areas
        .iter()
        .flat_map(|area| {
            let c0 = area.name();
            let data: Vec<(String, String, String)> = area
                .iter()
                .map(|(c1, c2)| (c0.clone(), c1.to_owned(), c2.to_owned()))
                .inspect(|line| debug!("{:?}", line))
                .collect();
            data
        })
        .collect();

    // Insert our header columns
    //
    flat.push_front(("Region".to_string(), "SAC".to_string(), "Label".to_string()));
    Ok(flat)
}

/// Output the final csv file, ensuring all strings are quoted for consistency
///
pub fn to_csv<T>(data: VecDeque<T>) -> Result<String>
where
    T: Serialize,
{
    trace!("Generating csv output…");
    // Prepare the writer
    //
    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .quote_style(QuoteStyle::Always)
        .from_writer(vec![]);

    // Insert data
    //
    data.iter().for_each(|rec| {
        wtr.serialize(rec).unwrap();
    });

    // Output final csv
    //
    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}
