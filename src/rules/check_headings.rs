use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;


rule_impl!(CheckHeadings, "Checks for inconsistent heading depths." => examples:
deep_heading,
"\
===== Deep Heading
Bla Blubb
",
"This heading is too deep.",
"== Normal Heading
Normal text
", "This heading is just fine.");
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

            // is heading too deep?
            if depth > settings.max_heading_depth {
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

            let current_depth = depth;
            let mut consistency_lint = None;

            {
                // find parent heading
                let parent = self.path.iter().rev().find(
                    |e| if let Element::Heading { .. } = ***e {**e != root} else {false}
                );

                if let Some(&&Element::Heading { depth, .. }) = parent {
                    if current_depth > depth + 1 {
                        consistency_lint = Some(Lint {
                            position: position.clone(),
                            explanation: "A sub heading should be exactly one level \
                                        deeper than its parent heading!".to_string(),
                            solution: format!("Reduce depth of this heading by {}.",
                                        current_depth - depth - 1),
                            explanation_long: "MFNF aims for a relatively shallow article structure. \
                                        To achieve this, the minimum heading level allowed is 2, \
                                        the maximum heading level is 4.".into(),
                            severity: Severity::Warning,
                        });
                    }
                }
            }

            if let Some(lint) = consistency_lint {
                self.push(lint);
            }
        }
        Ok(true)
    }
}
