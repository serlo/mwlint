use std::error;
use std::fmt;
use mediawiki_parser::ast;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
struct LinterError<'a> {
    position: ast::Span,
    message: &'a str,
    solution: &'a str,
    severity: LinterSeverity,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="lowercase")]
enum LinterSeverity {
    Info,
    Warning,
    Error,
}

impl<'a> fmt::Display for LinterError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl<'a> error::Error for LinterError<'a> {
    fn description(&self) -> &str {
        self.message
    }
}
