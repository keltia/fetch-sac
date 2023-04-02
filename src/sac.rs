use log::trace;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::sac::SAC::{Hex, Range};

use serde::{Deserialize, Serialize};

// ----------------------------------

/// Either  regular hex string or a range
///
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
                SAC::Hex(s) => s,
                SAC::Range(s) => s,
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

// ----------------------------------

/// One `Area` (group of countries, continent, etc.)
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Area {
    /// Name of the area
    pub label: String,
    /// List of codes
    pub list: HashMap<SAC, String>,
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
    pub fn add<T>(&mut self, code: T, label: &str) -> &mut Self
    where
        T: Into<SAC> + Display,
    {
        trace!("add({}, {})", code, label.to_owned());
        self.list.insert(code.into(), label.to_owned());
        self
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}", self.label, self.list)
    }
}
// ----------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[test]
    fn test_area_new() {
        let a = Area::new("foo");
        assert_eq!("foo", a.label);
        assert_eq!(HashMap::new(), a.list);
    }

    #[test]
    fn test_area_add() {
        let mut a = Area::new("foo");

        a.add("666", "Hell");
        assert_eq!("foo", a.label);
        assert!(a.list.get(&SAC::from("666")).is_some());
        assert_eq!("Hell", a.list.get(&SAC::from("666")).unwrap());
    }
}
