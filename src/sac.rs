use std::collections::HashMap;

use crate::sac::SAC::{Hex, Range};

use serde::{Deserialize, Serialize};

/// Either  regular hex string or a range
///
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SAC {
    /// Simple hex value
    Hex(String),
    /// Range of code, maybe use a real range type?
    Range(String),
}

impl SAC {
    /// Sometimes we have a range and not just a hex number
    ///  
    pub fn new(s: &str) -> Self {
        if s.contains("...") {
            Range(s.to_owned())
        } else {
            Hex(s.to_owned())
        }
    }
}

/// One entry, every `SAC` is in hex
///
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Code {
    /// Hex value (or range)
    pub sac: SAC,
    /// Country or part thereof
    pub label: String,
}

impl Code {
    /// Create a new `Code`
    ///
    pub fn new(sac: &str, label: &str) -> Self {
        Code {
            sac: SAC::new(sac),
            label: label.to_owned(),
        }
    }
}

/// One `Area` (group of countries, continent, etc.)
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Area {
    /// Name of the area
    pub label: String,
    /// List of codes
    pub list: HashMap<Code, String>,
}

impl Area {
    /// Create new instance
    ///
    pub fn new(s: &str) -> Self {
        Area {
            label: s.to_owned(),
            list: HashMap::new(),
        }
    }

    /// Add a code
    ///
    pub fn add(&mut self, code: Code, label: &str) -> &mut Self {
        self.list.insert(code.to_owned(), label.to_owned());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A4", SAC::Hex("A4".to_owned()))]
    #[case("00", SAC::Hex("00".to_owned()))]
    #[case("A0...C3", SAC::Range("A0...C3".to_owned()) )]
    fn test_sac_new(#[case] num: &str, #[case] sac: SAC) {
        assert_eq!(sac, SAC::new(num))
    }
}
