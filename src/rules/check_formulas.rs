use preamble::*;
use mwparser_utils::util::TexResult;
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
);


impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckFormulas<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        let checker = if let Some(ref checker) = settings.tex_checker {
            checker
        } else {
            return Ok(false)
        };

        if let Element::Formatted(ref formatted) = *root {
            if formatted.markup != MarkupType::Math {
                return Ok(true)
            }

            if let Some(&Element::Text(ref text)) = formatted.content.first() {
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
