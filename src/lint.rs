use std::error;
use std::fmt;
use mediawiki_parser::*;
use colored::*;


/// Specifies an issue identified by the linter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub struct Lint {
    /// Position in the source document.
    pub position: Span,
    /// Short, general explanation.
    pub explanation: String,
    /// Long explanation of the lint.
    pub explanation_long: String,
    /// Explains what to do about it.
    pub solution: String,
    /// Lint severity.
    pub severity: Severity,
    /// The lint kind.
    pub kind: LintKind,
}

/// Defines possible kinds of lints.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum LintKind {
    MaxHeadingDepthViolation,
    InconsistentHeadingHierarchy,
    DefinitionTermWithoutDef,
    DefinitionWithoutTerm,
    ListOneElement,
    ListMixedType,
}

/// Specifies examples for linter rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub struct Example {
    /// Example identifier.
    pub name: String,
    /// Example of a bad input.
    pub bad: String,
    /// Example of a good input.
    pub good: String,
    /// Explanation why the bad input is bad.
    pub bad_explanation: String,
    /// Explanation why the good input is good.
    pub good_explanation: String,
    /// The type of lint it should emitt.
    pub kind: LintKind,
}

/// The issue severity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Example: {}", self.name.blue())?;
        writeln!(f, "{}", self.bad_explanation)?;
        writeln!(f, "{}", self.bad)?;
        writeln!(f, "{}", self.good_explanation)?;
        write!(f, "{}", self.good)
    }
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message = format!(
            " at {}:{} to {}:{}: {}",
            self.position.start.line,
            self.position.start.col,
            self.position.end.line,
            self.position.end.col,
            self.explanation
        );
        let fancy = match self.severity {
            Severity::Info => format!("INFO: {} ({:?})", message, self.kind).blue(),
            Severity::Warning => format!("WARNING: {} ({:?})", message, self.kind).bright_yellow(),
            Severity::Error => format!("ERROR: {} ({:?})", message, self.kind).red(),
        };
        writeln!(f, "{}", fancy.bold())?;
        writeln!(f, "{}", self.explanation_long)?;
        write!(f, "{} {}", "try:".green().bold(), self.solution)
    }
}

impl error::Error for Lint {
    fn description(&self) -> &str {
        &self.explanation
    }
}
