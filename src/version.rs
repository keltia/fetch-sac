use clap::{crate_authors, crate_version};

use crate::cli::ABOUT;

/// Binary name, using a different binary name
pub(crate) const NAME: &str = env!("CARGO_BIN_NAME");
/// Binary version
pub(crate) const VERSION: &str = crate_version!();
/// Authors
pub(crate) const AUTHORS: &str = crate_authors!();

/// Display our version banner
///
#[inline]
pub fn version() -> String {
    format!("{}/{} by {}\n{}", NAME, VERSION, AUTHORS, ABOUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();

        assert!(v.starts_with(&format!("{NAME}/{VERSION}")));
    }
}
