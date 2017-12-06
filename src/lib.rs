extern crate mediawiki_parser;
extern crate serde;
#[macro_use]
extern crate serde_derive;

/// Provides linter result definitions.
pub mod error;
/// Various helper functions.
pub mod utils;
/// The checking functions themselves.
pub mod rules;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
