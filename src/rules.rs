use error::*;
use utils::*;
use mediawiki_parser::ast::*;
use settings::*;


build_rule_meta! {

/// Check heading depths.
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

/// Check list structure.
pub fn check_lists<'a>(
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<Lint>,
) {
    match root {
        &Element::List { ref content, .. } => {
            let first_kind = match content.first() {
                Some(&Element::ListItem { ref kind, .. }) => Some(kind.clone()),
                _ => None
            };

            if first_kind == Some(ListItemKind::Definition) {
                let err = Lint {
                    position: content.first().unwrap_or(root).get_position().clone(),
                    message: "A definition list should start with a definition \
                            term (;), not with definition text (:).".to_string(),
                    solution: "Prepend a definition term.".to_string(),
                    severity: Severity::Info,
                };
                lints.push(err);
            }

            for item in content {
                match item {
                    &Element::ListItem { ref kind, ref position, .. } => {
                        let matching = match first_kind {
                            Some(ListItemKind::Definition) => {
                                *kind == ListItemKind::DefinitionTerm ||
                                *kind == ListItemKind::Definition
                            },
                            Some(ListItemKind::DefinitionTerm) => {
                                *kind == ListItemKind::DefinitionTerm ||
                                *kind == ListItemKind::Definition
                            },
                            Some(ListItemKind::Ordered) => *kind == ListItemKind::Ordered,
                            Some(ListItemKind::Unordered) => *kind == ListItemKind::Unordered,
                            None => false,
                        };
                        if !matching {
                            let err = Lint {
                                position: position.clone(),
                                message: "List types (like ordered (#), unordered (*) \
                                          or definition lists (;/:) should not be mixed."
                                          .to_string(),
                                solution: "Split the list in two separate lists (line break) \
                                           or use a sublist.".to_string(),
                                severity: Severity::Warning,
                            };
                            lints.push(err);
                        }
                    },
                    _ => {
                        let err = Lint {
                            position: item.get_position().clone(),
                            message: "A list should only contain list items \
                                      in its content attribute.".to_string(),
                            solution: "This should not be produced by the parser. \
                                      Please file a bug at \
                                      https://github.com/vroland/mediawiki-peg-rust!".to_string(),
                            severity: Severity::Error,
                        };
                        lints.push(err);
                    }
                }
            }
        },
        _ => (),
    };
    lint_elem(&check_lists, root, path, settings, lints);
}


/// Verify templates are whitelisted.
pub fn check_template_whitelist<'a>(
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<Lint>,
) {
    match root {
        &Element::Template { ref name, ref position, .. } => {

            let template_name_err = Lint {
                position: position.clone(),
                message: "Template name is not text-only. This could make further \
                        processing more complicated.".to_string(),
                solution: "Maybe a flat template name is also suitable here.".to_string(),
                severity: Severity::Info,
            };

            if name.len() != 1 {
                lints.push(template_name_err);
            } else {
                let check_text = | text: &String, position: &Span, lints: &mut Vec<Lint> | {
                    if !settings.parameters.template_whitelist.contains(text) {

                        let err = Lint {
                            position: position.clone(),
                            message: format!("Template name '{}' \
                                    is not in template whitelist!", text),
                            solution: "Consider using a whitelisted template. If there \
                                    is none, think about adding a semantic template \
                                    to the whitelist instead of using only \
                                    visual markup!".to_string(),
                            severity: Severity::Warning,
                        };
                        lints.push(err);
                    }
                };
                match name.first() {
                    Some(&Element::Text { ref text, ref position }) => {
                        check_text(&text.trim().to_string(), position, lints);
                    },
                    Some(&Element::Paragraph { ref content, .. }) => {
                        if content.len() != 1 {
                            lints.push(template_name_err);
                        } else {
                            match content.first() {
                                Some(&Element::Text { ref text, ref position }) => {
                                    check_text(&text.trim().to_string(), position, lints);
                                },
                                _ => {
                                    lints.push(template_name_err);
                                }
                            }
                        }
                    },
                    _ => {
                        lints.push(template_name_err);
                    }
                }
            }
        },
        _ => (),
    };
    lint_elem(&check_template_whitelist, root, path, settings, lints);
}


}
