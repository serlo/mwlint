extern crate mediawiki_parser;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate config;


/// Provides linter result definitions.
pub mod error;
/// Various helper functions.
#[macro_use]
pub mod utils;
/// The checking functions themselves.
pub mod rules;
/// Structures for configuration of linter behaviour.
pub mod settings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
