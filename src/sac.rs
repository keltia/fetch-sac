use std::collections::btree_map::{IntoValues, Iter, Keys, Values, ValuesMut};
use log::trace;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::sac::SAC::{Hex, Range};

use serde::{Deserialize, Serialize};

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
    label: String,
    /// List of codes
    list: BTreeMap<SAC, String>,
}

impl Area {
    /// Create new instance
    ///
    pub fn new(s: &str) -> Self {
        Area {
            label: s.to_owned(),
            list: BTreeMap::new(),
        }
    }

    /// Name of the area
    ///
    #[inline]
    pub fn name(&self) -> String {
        self.label.to_owned()
    }

    /// Wrap `BtreeMap::get`
    ///
    #[inline]
    pub fn get<T>(&self, name: T) -> Option<&String>
    where T: Into<SAC>,
    {
        self.list.get(&name.into())
    }

    /// Wrap `is_empty()`
    ///
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Wrap `len()`
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Wrap `keys()`
    ///
    #[inline]
    pub fn keys(&self) -> Keys<'_, SAC, String> {
        self.list.keys()
    }

    /// Wrap `index_mut()`
    ///
    #[inline]
    pub fn index_mut<T>(&mut self, s: T) -> Option<&String>
    where T: Into<SAC>,
    {
        self.list.get(&s.into())
    }

    /// Wrap `values()`
    ///
    #[inline]
    pub fn values(&self) -> Values<'_, SAC, String> {
        self.list.values()
    }

    /// Wrap `values_mut()`
    ///
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, SAC, String> {
        self.list.values_mut()
    }

    /// Wrap `into_values()`
    ///
    #[inline]
    pub fn into_values(self) -> IntoValues<SAC, String> {
        self.list.into_values()
    }

    /// Wrap `contains_key()`
    ///
    #[inline]
    pub fn contains_key<T>(&self, s: T) -> bool
    where T: Into<SAC>,
    {
        self.list.contains_key(&s.into())
    }

    /// Wrap `iter()`
    ///
    #[inline]
    pub fn iter(&self) -> Iter<'_, SAC, String> {
        self.list.iter()
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

impl<'a> IntoIterator for &'a Area
{
    type Item = (&'a SAC, &'a String);
    type IntoIter = Iter<'a, SAC, String>;

    /// We can now do `sources.iter()`
    ///
    fn into_iter(self) -> Iter<'a, SAC, String> {
        self.list.iter()
    }
}

impl Index<&SAC> for Area {
    type Output = String;

    /// Wrap `index()`
    ///
    #[inline]
    fn index(&self, s: &SAC) -> &Self::Output
    {
        let me = self.list.get(s);
        me.unwrap()
    }
}

impl IndexMut<&SAC> for Area {
    /// Wrap `index_mut()`
    ///
    #[inline]
    fn index_mut(&mut self, s: &SAC) -> &mut Self::Output
    {
        let me = self.list.get_mut(s);
        if me.is_none() {
            self.list.insert(s.clone(), "".to_owned());
        }
        self.list.get_mut(s).unwrap()
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.list
            .iter()
            .map(|(sac, label)|
                format!("  {sac} = {label}")
            ).collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}\n{}", self.label, str)
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
        assert_eq!(BtreeMap::new(), a.list);
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
