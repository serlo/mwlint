use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;


/// Checks list structure.
#[derive(Debug, Default)]
pub struct CheckLists<'e> {
    pub path: Vec<&'e Element>,
    pub lints: Vec<Lint>,
}

rule_impl!("check lists", "list item types should make sense.", CheckLists);

impl<'e, 's> Traversion<'e, &'s RuleParameters> for CheckLists<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            parameters: &RuleParameters,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::List { ref content, .. } = *root {

            let first_kind;
            let first_position = content.first().unwrap_or(root).get_position();

            if let Some(&Element::ListItem { ref kind, .. }) = content.first() {
                 first_kind = kind.clone();
            } else {
                let err = Lint {
                    position: first_position.clone(),
                    message: "A list should only contain list items \
                                in its content attribute.".into(),
                    solution: "This should not be produced by the parser. \
                                Please file a bug at \
                                https://github.com/vroland/mediawiki-peg-rust!".into(),
                    severity: Severity::Error,
                };
                self.push(err);
                return Ok(false)
            };

            if first_kind == ListItemKind::Definition {
                let err = Lint {
                    position: first_position.clone(),
                    message: "A definition list should start with a definition \
                            term (;), not with definition text (:).".to_string(),
                    solution: "Prepend a definition term.".to_string(),
                    severity: Severity::Info,
                };
                self.push(err);
            }

            for item in content {
                if let Element::ListItem { ref kind, ref position, .. } = *item {
                    let matching = match first_kind {
                        ListItemKind::Definition => {
                            *kind == ListItemKind::DefinitionTerm ||
                            *kind == ListItemKind::Definition
                        },
                        ListItemKind::DefinitionTerm => {
                            *kind == ListItemKind::DefinitionTerm ||
                            *kind == ListItemKind::Definition
                        },
                        ListItemKind::Ordered => *kind == ListItemKind::Ordered,
                        ListItemKind::Unordered => *kind == ListItemKind::Unordered,
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
                        self.push(err);
                    }
                } else {
                    let err = Lint {
                        position: item.get_position().clone(),
                        message: "A list should only contain list items \
                                   in its content attribute.".into(),
                        solution: "This should not be produced by the parser. \
                                   Please file a bug at \
                                   https://github.com/vroland/mediawiki-peg-rust!".into(),
                        severity: Severity::Error,
                    };
                    self.push(err);
                }
            }
        };
        Ok(true)
    }
}
