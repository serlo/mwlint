extern crate mediawiki_parser;
extern crate mfnf_commons;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate colored;

/// Provides linter result definitions.
mod lint;
/// Structures for configuration of linter behaviour.
mod settings;
/// Data structures for defining rules.
#[macro_use]
mod rule;

/// common imports for rules.
mod preamble {
    pub use lint::{Lint, LintKind, Severity, Example};
    pub use rule::*;
    pub use mediawiki_parser::*;
    pub use settings::{Settings, RuleMeta};
    pub use std::io;
    pub use mfnf_commons::util::*;
}

/// The checking functions themselves.
mod rules;

pub use settings::{Settings};
pub use rule::{Rule, Checkable};
pub use lint::{Example, Lint, Severity};
pub use rules::*;
