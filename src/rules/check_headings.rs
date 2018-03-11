use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;


rule_impl!(CheckHeadings, "Checks for inconsistent heading depths." =>
deep_heading,
"\
===== Deep Heading
Bla Blubb
",
"This heading is too deep.",
"== Normal Heading
Normal text
",
"This heading is just fine."
);

impl<'e, 's> Traversion<'e, &'s Settings> for CheckHeadings<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let &Element::Heading {
            depth,
            ref position,
            ..
        } = root {

            let current_depth = depth;

            // is heading too deep?
            if current_depth > settings.max_heading_depth {
                let err = Lint {
                    position: position.clone(),
                    explanation: format!("A heading should not be deeper than level {}!",
                             settings.max_heading_depth),
                    explanation_long: "MFNF aims for a relatively shallow article structure. \
                                      To achieve this, the minimum heading level allowed is 2, \
                                      the maximum heading level is 4.".into(),
                    solution: "Change your article \
                               to have a more shallow structure.".into(),
                    severity: Severity::Warning,
                };
                self.push(err);
            }
            // is heading depth appropriate?
            for elem in self.path.clone().iter().rev() {
                if let &Element::Heading { depth, .. } = *elem {
                    if current_depth > depth + 1 {
                        let err = Lint {
                            position: position.clone(),
                            explanation: "A sub heading should be exactly one level \
                                        deeper than its parent heading!".to_string(),
                            solution: format!("Reduce depth of this heading by {}.",
                                        current_depth - depth - 1),
                            explanation_long: "MFNF aims for a relatively shallow article structure. \
                                      To achieve this, the minimum heading level allowed is 2, \
                                      the maximum heading level is 4.".into(),
                            severity: Severity::Warning,
                        };
                        self.push(err);
                    }
                    break;
                }
            }
        }
        Ok(true)
    }
}
