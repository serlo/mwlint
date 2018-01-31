use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;


/// Tests for inconsistent heading depths.
#[derive(Debug, Default)]
pub struct CheckHeadings<'e> {
    pub path: Vec<&'e Element>,
    pub lints: Vec<Lint>,
}

rule_impl!("check headings", "headings should have consistent depths.", CheckHeadings);

impl<'e, 's> Traversion<'e, &'s RuleParameters> for CheckHeadings<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            parameters: &RuleParameters,
            _: &mut io::Write) -> io::Result<bool> {

        if let &Element::Heading {
            depth,
            ref position,
            ..
        } = root {

            let current_depth = depth;

            // is heading too deep?
            if current_depth > parameters.max_heading_depth {
                let err = Lint {
                    position: position.clone(),
                    message: format!("A heading should not be deeper than level {}!",
                             parameters.max_heading_depth),
                    solution: "Change your article \
                               to have a more shallow structure.".to_string(),
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
                            message: "A sub heading should be exactly one level \
                                        deeper than its parent heading!".to_string(),
                            solution: format!("Reduce depth of this heading by {}.",
                                        current_depth - depth - 1),
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
