use preamble::*;
use lint::{Example, Lint};

rule_impl!(CheckHtml, "Hints for text elements."
=> examples:
    illegal_span,
    "<big>big text</big>",
    "<big> is not allowed for this Projekt because it lacks portability.",
    "'''fat text'''",
    "Bold text highlights just as well."
    => LintKind::IllegalHtml
);


impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckHtml<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::HtmlTag(ref html) = *root {

            if !settings.html_whitelist.contains(&html.name.trim().to_lowercase()) {
                let html_lint = Lint {
                    position: html.position.clone(),
                    explanation: format!("\"{}\" is not allowed for this project.",
                                         &html.name),
                    explanation_long:
                        "Custom HTML tags are usually not well portable and \
                        can impair consistency. <span> for example would allow \
                        any CSS markup.".into(),
                    solution: "Use MediaWiki markup or allowed templtes.".into(),
                    severity: Severity::Error,
                    kind: LintKind::IllegalHtml,
                };
                self.push(html_lint);

            }
        }
        Ok(true)
    }
}
