use preamble::*;
use mwparser_utils::TexResult;
use lint::{Example, Lint};

rule_impl!(CheckFormulas, "Verify math formulas."
=> examples:
    math_syntax_error,
    "<math>\\frac{1}{2</math>",
    "There is an error in the math forumla. In this formula for example is \
    the closing tag '}' missing.",
    "<math>\\frac{1}{2}</math>",
    "This formula is syntactically correct."
    => LintKind::MathSyntaxError
;
    math_lexing_error,
    "<math>\\[ x^2 \\]</math>",
    "The formula contains invalid characters. The chracter `[` is for example \
    not allowed in LaTeX for MediaWiki.",
    "<math>x^2</math>",
    "This formula does not contain invalid characters."
    => LintKind::MathLexingError
;
    math_unknown_function,
    "<math>\\badfrac{1}{2}</math>",
    "The macro `badfrac` is not defined in MediaWiki LaTeX",
    "<math>\\frac{1}{2}</math>",
    "`frac` is a valid LaTeX macro."
    => LintKind::MathUnknownFunction
;
    righarrow_to_implies,
    "<math>A\\Rightarrow B</math>",
    "Rightarrow is not semantic and usually means implies.",
    "<math>A\\implies B</math>",
    "`implies` is correctly used. You may need to use `\\implies{}` before `&`."
    => LintKind::DeprecatedRightarrow
;
    leftrightarrow_to_iff,
    "<math>A\\Leftrightarrow B</math>",
    "Leftrightarrow is not semantic and usually means iff.",
    "<math>A\\implies B</math>",
    "`iff` is correctly used. Iff means \"if and only if\"."
    => LintKind::DeprecatedLeftrightarrow
);


impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckFormulas<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::Formatted(ref formatted) = *root {
            if formatted.markup != MarkupType::Math {
                return Ok(true)
            }

            if let Some(&Element::Text(ref text)) = formatted.content.first() {

                if text.text.contains("\\Rightarrow") {
                    let arrow_lint = Lint {
                        position: text.position.clone(),
                        explanation: "\\Rightarrow should not be used \
                            in math markup any more.".into(),
                        explanation_long:
                            "\\implies should be used instead of \\Rightarrow, \
                            because it conveys more semantic meaning. Sometimes \
                            \\implies{} works better before `&`.".into(),
                        solution: "Replace \\Rightarrow by \
                                   \\implies or \\implies{}.".into(),
                        severity: Severity::Warning,
                        kind: LintKind::DeprecatedRightarrow,
                    };
                    self.push(arrow_lint);
                }

                if text.text.contains("\\Leftrightarrow") {
                    let arrow_lint = Lint {
                        position: text.position.clone(),
                        explanation: "\\Leftrightarrow should not be used \
                            in math markup any more.".into(),
                        explanation_long:
                            "\\iff should be used instead of \\Leftrightarrow, \
                            because it conveys more semantic meaning. Iff is mathmatical \
                            speak for \"if and only if\".".into(),
                        solution: "Replace \\Leftrightarrow by \\iff.".into(),
                        severity: Severity::Warning,
                        kind: LintKind::DeprecatedLeftrightarrow,
                    };
                    self.push(arrow_lint);
                }

                let checker = if let Some(ref checker) = settings.tex_checker {
                    checker
                } else {
                    return Ok(false)
                };

                let error_cause = match checker.check(&text.text) {
                    TexResult::Ok(_) => None,
                    TexResult::SyntaxError => Some (
                        ( "This formula is not a valid LaTeX formula. You \
                           need to correct it.".into(),
                        LintKind::MathSyntaxError)
                    ),
                    TexResult::LexingError => Some (
                        ( "This formula contains characters which are not \
                           allowed in LaTeX for MediaWiki. You need to delete \
                           the unallowed characters.".into(),
                        LintKind::MathLexingError)
                    ),
                    TexResult::UnknownFunction(f) => Some (
                        ( format!("The macro `{}` is not known in LaTeX for \
                                   MediaWiki or is not allowed in formulas. \
                                   You need to correct the macro name or \
                                   to change your formula.", &f),
                        LintKind::MathUnknownFunction)
                    ),
                    TexResult::UnknownError => Some(
                        ( "An unknown error occured with this formula.".into(),
                        LintKind::MathUnknownFunction)
                    )
                };

                if let Some(error) = error_cause {
                    let err_lint = Lint {
                        position: text.position.clone(),
                        explanation: error.0,
                        explanation_long:
                            "Only a subset of LaTeX with some additional \
                             macros is allowed in MediaWiki. This formula \
                             does not result in a correct LaTeX output.".into(),
                        solution: "Only use LaTeX code allowed by the \
                                   MediaWiki Software.".into(),
                        severity: Severity::Error,
                        kind: error.1
                    };
                    self.push(err_lint);
                }
            }
        }
        Ok(true)
    }
}
