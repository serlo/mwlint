use error::*;
use utils::*;
use mediawiki_parser::ast::*;
use settings::*;


build_rule_meta! {

/// TODO: Check heading depths.
pub fn check_heading_depths<'a>(
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<Lint>,
) {
    match root {
        &Element::Heading {
            depth,
            ref position,
            ..
        } => {

            let current_depth = depth;

            // is heading too deep?
            if current_depth > settings.parameters.max_heading_depth {
                let err = Lint {
                    position: position.clone(),
                    message: format!("A heading should not be deeper than level {}!",
                             settings.parameters.max_heading_depth),
                    solution: "Re-structure your article \
                               structure to have a more shallow structure.".to_string(),
                    severity: Severity::Warning,
                };
                lints.push(err);
            }
            // is heading depth appropriate?
            for elem in path.iter().rev() {
                match *elem {
                    &Element::Heading { depth, .. } => {
                        if current_depth > depth + 1 {
                            let err = Lint {
                                position: position.clone(),
                                message: "A sub heading should be exactly one level \
                                          deeper than its parent heading!".to_string(),
                                solution: format!("Reduce depth of this heading by {}.",
                                          current_depth - depth - 1),
                                severity: Severity::Warning,
                            };
                            lints.push(err);
                        }
                        break;
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    };
    lint_elem(&check_heading_depths, root, path, settings, lints);
}

/// Documentation 1
pub fn test_rule1<'a>(
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<Lint>,
) {}

}
