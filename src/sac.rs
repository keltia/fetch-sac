use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::sac::SAC::{Hex, Range};

// ----------------------------------

/// Either  regular hex string or a range
///
#[derive(Clone, Debug, PartialOrd, Ord, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SAC {
    /// Simple hex value
    Hex(String),
    /// Range of code, maybe use a real range type?
    Range(String),
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
                SAC::Empty => "",
                Hex(s) => s,
                Range(s) => s,
            }
        )
    }
}

impl From<&str> for SAC {
    /// Easier to have than direct ::from()
    ///
    fn from(value: &str) -> Self {
        if value.contains("...") {
            Range(value.to_owned())
        } else {
            Hex(value.to_owned())
        }
    }
}

impl From<usize> for SAC {
    /// Easier to have than direct ::from()
    ///
    fn from(value: usize) -> Self {
        Hex(format!("{:02X}", value))
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

    #[rstest]
    #[case("A4", SAC::Hex("A4".to_owned()))]
    #[case("00", SAC::Hex("00".to_owned()))]
    #[case("A0...C3", SAC::Range("A0...C3".to_owned()) )]
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
