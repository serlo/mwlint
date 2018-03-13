use preamble::*;

rule_impl!(CheckLists, "Checks for malformed lists"
=> examples:
    definition_term_without_def,
    "; term 1\n\
        ; term 2",
    "The definitions of `term 1` and `term 2` are missing.",
    "; term 1\n\
        : definition 1\n\
        ; term 2\n\
        : definition 2",
    "The terms `term 1` and `term 2` are followed by their definitions."
    => LintKind::DefinitionTermWithoutDef
;
    definition_term_single,
    "; term 1\n",
    "The definition of `term 1` is missing.",
    "; term 1\n\
        : definition 1\n",
    "The terms `term 1` is followed by its definition."
    => LintKind::DefinitionTermWithoutDef
;
    definition_without_term,
    ": definition 1\n",
    "The term for definition `definition 1` is missing.",
    "; term 1\n\
        : definition 1\n",
    "The term `term 1` is followed by its definition."
    => LintKind::DefinitionWithoutTerm
;
    multiple_defs_without_term,
    "; term 1\n\
        : definition 1\n\
        : definition 2\n",
    "The term for `definition 2` is missing.",
    "; term 1\n\
        : definition 1\n\
        ; term 2\n\
        : definition 2\n",
    "The terms `term 1` and `term 2` are followed by their definition."
    => LintKind::DefinitionWithoutTerm
;
    list_one_item,
    "* <math>1+1=3</math>\n",
    "This is used to indent a single item. Semantically, this should not be a list.",
    "{{formula|<math>1+1=3</math>}}",
    "Instead, the formula template can be used to describe a distinct mathmatical fact."
    => LintKind::ListOneElement
;
    mixed_list,
    "* item one\n\
        * item two\n\
        # item three\n",
    "`item three` is ordered, but every other element is unordered.",
    "* item one\n\
        * item two\n\
        * item three\n",
    "The list type is consistent."
    => LintKind::ListMixedType
;
    mixed_sublist,
    "* item one\n\
        * item two\n\
        * item three\n\
        *; sub def term\n\
        *: sub def text\n\
        ** some more explanation.\n",
    "The top level list is consistent, but a sublists contains definitions and unordered items.",
    "* item one\n\
        * item two\n\
        * item three\n\
        *; sub def term\n\
        *: definition with longer explanation.\n\
        * item three (alternative): {{:Mathe für Nicht-Freaks: Vorlage:Definition| title=...}}\n",
    "Keep the sublist konsistent by either writing a longer definition or \
    using a sematic template."
    => LintKind::ListMixedType
);

fn term_without_def(
    position: &Span,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: "A defined term (;) must be followed by its definition (:)!".into(),
        explanation_long:
            "The ; and : elements mark definition lists. They should only be used \
            for lists of a terms and their corresponding definition. Using it for \
            indentation defeats its semantic meaning.".into(),
        solution: "Add a definition item (:) after the term.".into(),
        severity: Severity::Warning,
        kind: LintKind::DefinitionTermWithoutDef,
    }
}

fn def_without_term(
    position: &Span,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: "A definition (:) must be preceded by a definition term (;)!".into(),
        explanation_long:
            "The ; and : elements mark definition lists. They should only be used \
            for lists of a terms and their corresponding definition. Using it for \
            indentation defeats its semantic meaning.".into(),
        solution: "Add a term (;) before the definition.".into(),
        severity: Severity::Warning,
        kind: LintKind::DefinitionWithoutTerm,
    }
}

fn list_one_element(
    position: &Span,
) -> Lint {
     Lint {
        position: position.clone(),
        explanation: "Lists with one element are useless!".into(),
        explanation_long:
            "A list is a collection of elements. \
            But this list only contains one list element. Maybe some items are missing \
            or some other kind of markup should be used.".into(),
        solution:
            "Either delete this list or think of its semantic purpose \
            to find alternatives.".into(),
        severity: Severity::Info,
        kind: LintKind::ListOneElement,
    }
}

fn list_mixed_type(
    position: &Span,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: "Lists kinds (unordered, ordered, definition) should not be mixed!".into(),
        explanation_long:
            "Mediawiki allows mixed types of list items. This is discouraged, as it does \
            not convey any universally understood meaning.".into(),
        solution:
            "Use consistent item types or split into several lists.".into(),
        severity: Severity::Error,
        kind: LintKind::ListMixedType,
    }
}

// factor list item kinds in semantic groups.
fn term_to_def(kind: &ListItemKind) -> ListItemKind {
    if let ListItemKind::DefinitionTerm = *kind {
        ListItemKind::Definition
    } else {
        kind.clone()
    }
}

impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckLists<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            _: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::List {
            ref position,
            ref content,
             ..
        } = *root {

            if content.len() == 1 {
                self.push(list_one_element(position));
            }

            let mut previous_kind = None;
            let mut previous_item: Option<&Element> = None;
            for (index, item) in content.iter().enumerate() {
                if let Element::ListItem { ref kind, .. } = *item {
                    let is_def = *kind == ListItemKind::Definition;
                    let is_term = *kind == ListItemKind::DefinitionTerm;
                    let is_last = index == content.len() - 1;
                    let is_first = index == 0;
                    let prev_is_term = previous_kind == Some(ListItemKind::DefinitionTerm);

                    if (prev_is_term && !is_def) || (is_term && is_last) {
                        let mut position = if is_last {
                            item.get_position()
                        } else {
                            previous_item.unwrap_or(item).get_position()
                        };
                        self.push(term_without_def(position));
                    }

                    if (is_def && !prev_is_term) || (is_def && is_first) {
                        let position = item.get_position();
                        self.push(def_without_term(position));
                    }

                    if let Some(prev) = previous_kind {
                        let fac_kind = term_to_def(kind);
                        let fac_prev = term_to_def(&prev);

                        if fac_kind != fac_prev {
                            self.push(list_mixed_type(item.get_position()));
                        }
                    }

                    previous_kind = Some(kind.clone());
                    previous_item = Some(item);
                }
            }
        };
        Ok(true)
    }
}
