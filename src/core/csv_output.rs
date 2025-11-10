//! Module dealing data preparation for CSV output
//!
//! This module provides functionality to prepare and format data for CSV output.
//! It handles flattening of nested data structures and ensures consistent CSV formatting
//! with quoted strings.
//!

use anyhow::Result;
use csv::{QuoteStyle, WriterBuilder};
use log::{debug, trace};
use serde::Serialize;
use std::collections::VecDeque;

use crate::core::Area;

/// Prepares data for CSV output by flattening the area structure.
///
/// # Arguments
///
/// * `areas` - A slice of Area objects containing the data to be prepared
///
/// # Returns
///
/// * `Result<VecDeque<(String, String, String)>>` - A deque containing tuples of (Region, SAC, Label)
///
/// This function takes an array of areas and generates a flattened structure suitable for CSV output.
/// It creates rows where each row contains:
/// 1. The area name (region)
/// 2. The SAC code
/// 3. The corresponding label
///
/// The first row contains the header names: "Region", "SAC", "Label"
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

/// Converts prepared data into CSV format with consistent string quoting.
///
/// # Arguments
///
/// * `data` - A VecDeque of serializable data to be written as CSV
///
/// # Returns
///
/// * `Result<String>` - The formatted CSV data as a string
///
/// This function takes the prepared data and converts it to CSV format where:
/// - All fields are quoted (for consistency)
/// - Fields are comma-delimited
/// - The first row is treated as headers
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
