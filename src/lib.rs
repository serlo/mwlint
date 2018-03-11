extern crate mediawiki_parser;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate colored;


/// Provides linter result definitions.
mod error;
/// Various helper functions.
#[macro_use]
mod utils;
/// The checking functions themselves.
mod rules;
/// Structures for configuration of linter behaviour.
mod settings;

pub use settings::{Settings};
pub use utils::{Rule, Checkable};
pub use error::{Example, Lint, Severity};
pub use rules::*;
