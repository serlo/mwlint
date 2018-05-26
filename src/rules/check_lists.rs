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
    "; term 1",
    "The definition of `term 1` is missing.",
    "; term 1\n\
     : definition 1",
    "The terms `term 1` is followed by its definition."
    => LintKind::DefinitionTermWithoutDef
;
    math_usage_definition_term,
    ":<math>x^2</math>",
    "A definition list is used for indenting a mathematical formula. This \
     would result in a definition list without a definition term. The proper \
     way to indent a formula is by using the {{Formel|...}} template.",
    "{{Formel|<math>x^2</math>}}",
    "The formula is defined inside the apropriate template for math formulas."
    => LintKind::DefinitionWithoutTerm
;
    definition_without_term,
    ": definition 1",
    "The term for definition `definition 1` is missing.",
    "; term 1\n\
     : definition 1",
    "The term `term 1` is followed by its definition."
    => LintKind::DefinitionWithoutTerm
;
    multiple_defs_without_term,
    "; term 1\n\
     : definition 1\n\
     : definition 2",
    "The term for `definition 2` is missing.",
    "; term 1\n\
     : definition 1\n\
     ; term 2\n\
     : definition 2",
    "`defintion 2` follows the definition term `term 2`."
    => LintKind::DefinitionWithoutTerm
;
    list_one_item_split_lists,
    "* item 1\n\
     \n\
     * item 2",
    "An empty line ends a list. Thus we have here two seperate lists. In \
     order to have one list we need to delete the empty line between the two \
     lists.",
    "* item 1\n\
     * item 2",
    "We only have list with two items since there is no empty line betweens \
     the lines."
    => LintKind::ListOneElement
;
    list_one_item_important_paragraph,
    "Some paragraph.\n\
     \n\
     * An important fact.\n\
     \n\
     Another paragraph.",
    "Here the list asteriks `*` is used to format a text. However, the \
     formated element is no list. There another format which is semantically \
     more propriate should be used.",
    "Some paragraph.\n\
     \n\
     {{-|An important fact.}}\n\
     \n\
     Another paragraph.",
    "Instead of a list the template {{-|...}} for marking important paragraphs
     is used for formating the important paragraph."
    => LintKind::ListOneElement
;
    mixed_list,
    "* item one\n\
     * item two\n\
     # item three",
    "`item three` is ordered, but every other list item is unordered. Ordered \
     list items are marked with `#` and unordered list items with `*`.",
    "* item one\n\
     * item two\n\
     * item three",
    "The list type is consistent between all list items."
    => LintKind::ListMixedType
;
    mixed_sublist,
    "* item one\n\
     * item two\n\
     * item three\n\
     *# sub item 1\n\
     ** sub item 2",
    "The top level list is consistent. However the sublists contains a \
     ordered and unordered items.",
    "* item one\n\
     * item two\n\
     * item three\n\
     *# sub def term\n\
     *# definition with longer explanation.",
    "Keep the sublist consistent by using only one sublist type. Here only \
     ordered list items are used."
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
            "Do you need a list here? For lists with longer paragraphs, \
             use the `list`-template!".into(),
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
        *kind
    }
}

impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckLists<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            _: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::List(ref list) = *root {

            if list.content.len() == 1 {
                self.push(list_one_element(&list.position));
            }

            let mut previous_kind = None;
            let mut previous_item: Option<&ListItem> = None;
            for (index, item) in list.content.iter().enumerate() {
                if let Element::ListItem(ref item) = *item {
                    let is_def = item.kind == ListItemKind::Definition;
                    let is_term = item.kind == ListItemKind::DefinitionTerm;
                    let is_last = index == list.content.len() - 1;
                    let is_first = index == 0;
                    let prev_is_term = previous_kind == Some(ListItemKind::DefinitionTerm);

                    if (prev_is_term && !is_def) || (is_term && is_last) {
                        let mut position = if is_last {
                            &item.position
                        } else {
                            &previous_item.unwrap_or(item).position
                        };
                        self.push(term_without_def(position));
                    }

                    if (!prev_is_term || is_first) && is_def {
                        self.push(def_without_term(&item.position));
                    }

                    if let Some(prev) = previous_kind {
                        let fac_kind = term_to_def(&item.kind);
                        let fac_prev = term_to_def(&prev);

                        if fac_kind != fac_prev {
                            self.push(list_mixed_type(&item.position));
                        }
                    }

                    previous_kind = Some(item.kind);
                    previous_item = Some(item);
                }
            }
        };
        Ok(true)
    }
}
