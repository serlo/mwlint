use std::error;
use std::fmt;
use mediawiki_parser::ast;


/// Specifies an issue identified by the linter.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
pub struct Lint<'a> {
    position: ast::Span,
    message: &'a str,
    solution: &'a str,
    severity: Severity,
}


/// The issue severity.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl<'a> fmt::Display for Lint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl<'a> error::Error for Lint<'a> {
    fn description(&self) -> &str {
        self.message
    }
}
