extern crate mediawiki_parser;
extern crate mfnf_template_spec;
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
    pub use crate::lint::{Example, Lint, LintKind, Severity};
    pub use crate::rule::*;
    pub use crate::settings::{RuleMeta, Settings};
    pub use mediawiki_parser::*;
    pub use mwparser_utils::*;
    pub use std::io;
    pub use std::path::PathBuf;
}

/// The checking functions themselves.
mod rules;

pub use crate::lint::{Example, Lint, Severity};
pub use crate::rule::{Checkable, Rule};
pub use crate::rules::*;
pub use crate::settings::Settings;

/// Applies transformations to normalize the input tree.
pub fn normalize(
    mut root: mediawiki_parser::Element,
    _settings: &settings::Settings,
) -> mediawiki_parser::transformations::TResult {
    root = mwparser_utils::transformations::convert_template_list(root)?;
    Ok(root)
}
