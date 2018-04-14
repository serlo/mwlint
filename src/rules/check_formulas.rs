use preamble::*;
use std::process::{Command};
use lint::{Example, Lint};

rule_impl!(CheckFormulas, "Verify math formulas."
=> examples:
    math_syntax_error,
    "<math>\\frac{1}{2</math>",
    "This formula is missing a closing tag.",
    "<math>\\frac{1}{2}</math>",
    "This formula is syntactically correct."
    => LintKind::MathSyntaxError
;
    math_lexing_error,
    "<math>\\[ bla</math>",
    "Some inputs contain invalid tokens.",
    "<math>bla</math>",
    "This input does not contain weired characters."
    => LintKind::MathLexingError
;
    math_unknown_function,
    "<math>\\heyho{1+2}</math>",
    "The function `heyho` is not defined in MediaWiki LaTeX",
    "<math>\\frac{1}{2}</math>",
    "The `frac` function is known."
    => LintKind::MathUnknownFunction
);


impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckFormulas<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if settings.texvccheck_path.is_empty() {
            return Ok(false)
        }

        if let &Element::Formatted {
            ref content,
            ref markup,
            ..
        } = root {
            if *markup != MarkupType::Math {
                return Ok(true)
            }

            if let Some(&Element::Text {
                ref position,
                ref text
            }) = content.first() {
                let mut results = check_formula(text, position, settings);
                for lint in results.drain(..) {
                    self.push(lint);
                }
            }
        }
        Ok(true)
    }
}

/// Check a Tex formula, return normalized version or error
/// TODO: unify this with mfnf-export
fn check_formula(
    content: &str,
    position: &Span,
    settings: &Settings
) -> Vec<Lint> {

    let mut result = vec![];

    let checked = texvccheck(content, settings);
    let error_cause = match checked.chars().next() {
        Some('+') => None,
        Some('S') => Some( ("syntax error".into(), LintKind::MathSyntaxError) ),
        Some('E') => Some( ("lexing error".into(), LintKind::MathLexingError) ),
        Some('F') => Some( ( format!("unknown function `{}`",
            checked.chars().skip(1).collect::<String>()), LintKind::MathUnknownFunction) ),
        Some('-') => Some( ("other error".into(), LintKind::MathUnknownError) ),
        None => Some( ("empty string".into(), LintKind::MathUnknownError) ),
        _ => Some( ("unknown error".into(), LintKind::MathUnknownError ) ),
    };

    if let Some(error) = error_cause {
        let err_lint = Lint {
            position: position.clone(),
            explanation: format!("This formula is invalid: {}", error.0),
            explanation_long:
                "Only a subset of LaTeX with some additional macros is \
                 allowed in MediaWiki markup. This formula does not result \
                 in a correct LaTeX output.".into(),
            solution: format!("Only use LaTeX code allowed by MediaWiki!"),
            severity: Severity::Error,
            kind: error.1
        };
        result.push(err_lint);
    }

    return result;
}

/// Call the external program `texvccheck` to check a Tex formula
fn texvccheck(formula: &str, settings: &Settings) -> String {
    let output = Command::new(&settings.texvccheck_path)
        .arg(formula)
        .output();

    let res = if let Ok(output) = output {
        output.stdout
    } else {
        vec!['-' as u8]
    };

    String::from_utf8(res).unwrap_or("-".into())
}
