use std::error;
use std::fmt;
use mediawiki_parser::ast;
use colored::*;


/// Specifies an issue identified by the linter.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub struct Lint {
    pub position: ast::Span,
    pub message: String,
    pub solution: String,
    pub severity: Severity,
}


/// The issue severity.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message = format!(
            " at {}:{} to {}:{}: {}",
            self.position.start.line,
            self.position.start.col,
            self.position.end.line,
            self.position.end.col,
            self.message
        );
        let fancy = match self.severity {
            Severity::Info => format!("INFO: {}", message).blue(),
            Severity::Warning => format!("WARNING: {}", message).bright_yellow(),
            Severity::Error => format!("ERROR: {}", message).red(),
        };
        writeln!(f, "{}", fancy.bold())?;
        writeln!(f, "{} {}", "try:".green().bold(), self.solution)
    }
}

impl error::Error for Lint {
    fn description(&self) -> &str {
        &self.message
    }
}
