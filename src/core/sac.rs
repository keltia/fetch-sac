//! Module which implements a model to represent SAC codes and their variations.
//!
//! The `Area` struct is using `String` as its key to avoid painful JSON issues which would
//! force me to implement a custom serializer/deserializer for the `SAC` enum.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

// ----------------------------------

#[allow(clippy::upper_case_acronyms)]

/// Either  regular hex string or a range
///
/// Represents a System Area Code (SAC) which can be either a single hex value,
/// a range of values, or empty.
/// 
#[derive(Clone, Debug, PartialOrd, Ord, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SAC {
    /// Single hexadecimal value stored as a string
    Hex(String),
    /// Range of SAC codes with lower and upper bounds
    Range { lo: usize, hi: usize },
    /// Represents an empty or undefined SAC code
    Empty,
}

impl SAC {
    /// Creates a new empty SAC code.
    ///
    /// # Returns
    /// Returns a `SAC::Empty` variant.
    pub fn new() -> Self {
        SAC::Empty
    }
}

impl Default for SAC {
    fn default() -> Self {
        SAC::new()
    }
}

impl Display for SAC {
    /// Set the default formatter
    ///
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SAC::Empty => "".to_owned(),
                SAC::Hex(s) => s.to_string(),
                SAC::Range { lo, hi } => format!("{:02X}...{:02X}", lo, hi),
            }
        )
    }
}

impl From<&str> for SAC {
    /// Converts a string slice into a SAC code.
    ///
    /// # Parameters
    /// - `value`: A string slice that can be either a hex value or a range in format "XX...YY"
    ///
    /// # Returns
    /// Returns either a `SAC::Hex` or `SAC::Range` variant depending on the input format.
    /// 
    fn from(value: &str) -> Self {
        if value.contains("...") {
            let val: Vec<&str> = value.split("...").collect();
            let lo = usize::from_str_radix(val[0], 16).unwrap();
            let hi = usize::from_str_radix(val[1], 16).unwrap();
            SAC::Range { lo, hi }
        } else {
            SAC::Hex(value.to_owned())
        }
    }
}

impl From<usize> for SAC {
    /// Converts a usize into a SAC hex code.
    ///
    /// # Parameters
    /// - `value`: A usize value to be converted into a hex representation
    ///
    /// # Returns
    /// Returns a `SAC::Hex` variant with the value formatted as a two-digit hex string.
    /// 
    fn from(value: usize) -> Self {
        SAC::Hex(format!("{:02X}", value))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_sac_new() {
        let sac = SAC::new();
        assert_eq!(SAC::Empty, sac);
    }

    #[test]
    fn test_sac_default() {
        let sac = SAC::default();
        assert_eq!(SAC::Empty, sac);
    }

    #[rstest]
    #[case("A4", SAC::Hex("A4".to_owned()))]
    #[case("00", SAC::Hex("00".to_owned()))]
    #[case("A0...C3", SAC::Range {lo: 160, hi: 195})]
    fn test_sac_from_str(#[case] num: &str, #[case] sac: SAC) {
        assert_eq!(sac, SAC::from(num))
    }

    #[rstest]
    #[case(164, SAC::Hex("A4".to_owned()))]
    #[case(0, SAC::Hex("00".to_owned()))]
    fn test_sac_from_usize(#[case] num: usize, #[case] sac: SAC) {
        assert_eq!(sac, SAC::from(num))
    }
}
