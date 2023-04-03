//! Module which implement a model to represent SAC codes anbd their variations.
//!
//! The `Area` struct is using `String`  as its key to avoid painful JSON issues which would
//! force me to implement a custom serializer/deserializer for the `SAC` enum.
//!

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

// ----------------------------------

/// Either  regular hex string or a range
///
#[derive(Clone, Debug, PartialOrd, Ord, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SAC {
    /// Simple hex value
    Hex(String),
    /// Range of codes
    Range { lo: usize, hi: usize },
    /// Guess
    Empty,
}

impl SAC {
    /// Create an Empty code
    ///
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
    /// Easier to have than direct ::from()
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
    /// Easier to have than direct ::from()
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
