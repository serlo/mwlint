use std::error;
use std::fmt;
use mediawiki_parser::ast;


/// Specifies an issue identified by the linter.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
pub struct Lint {
    pub position: ast::Span,
    pub message: String,
    pub solution: String,
    pub severity: Severity,
}


/// The issue severity.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.message)?;
        writeln!(f, "try: {}", self.solution)
    }
}

impl error::Error for Lint {
    fn description(&self) -> &str {
        &self.message
    }
}
