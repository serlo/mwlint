extern crate mediawiki_parser;
extern crate mwparser_utils;
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
    pub use mediawiki_parser::*;
    pub use lint::{LintKind, Lint, Severity, Example};
    pub use rule::*;
    pub use settings::{Settings, RuleMeta};
    pub use std::io;
    pub use mwparser_utils::util::*;
}

/// The checking functions themselves.
mod rules;

pub use settings::{Settings};
pub use rule::{Rule, Checkable};
pub use lint::{Example, Lint, Severity};
pub use rules::*;

/// Applies transformations to normalize the input tree.
pub fn normalize(mut root: mediawiki_parser::Element,
                 _settings: &settings::Settings)
-> mediawiki_parser::transformations::TResult {

    root = mwparser_utils::transformations::convert_template_list(root)?;
    Ok(root)
}
