use preamble::*;

rule_impl!(CheckHeadings, "Checks for erroneous headings."
=> examples:
    deep_heading,
    "===== deep heading\n",
    "This heading is too deep.",
    "== normal heading\n",
    "This heading is just fine."
    => LintKind::MaxHeadingDepthViolation
;
    inconsistent_heading,
    "== top level\n\
    ==== low level\n",
    "The lower level heading is two levels deeper than its parent.",
    "== top level\n\
    === low level\n",
    "The heading hierarchy is consistent."
    => LintKind::InconsistentHeadingHierarchy
);


fn max_depth_lint(
    settings: &Settings,
    position: &Span
) -> Lint {
    let max = settings.max_heading_depth;
    Lint {
        position: position.clone(),
        explanation: format!(
            "A heading should not be deeper than level {}!", max),
        explanation_long: format!(
            "MFNF aims for a relatively shallow article structure. \
            To achieve this, the minimum heading level allowed is 2, \
            the maximum heading level is {}.", max),
        solution:
            "Change your article to have a more shallow structure.".into(),
        severity: Severity::Warning,
        kind: LintKind::MaxHeadingDepthViolation,
    }
}

fn inconsistent_hierarchy_lint(
    position: &Span,
    diff: usize
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: "A sub heading should be exactly one level \
                        deeper than its parent heading!".into(),
        explanation_long: format!(
            "If a heading has a higher heading than a previous heading, \
            it is considered a sub heading of this heading. Thus, headings are \
            make up a hierarchy. But a heading more than one level deeper than \
            its parent makes no semantic sense. Heading levels should not be used \
            to do text formatting!"),
        solution: format!("Reduce depth of this heading by {}.", diff),
        severity: Severity::Warning,
        kind: LintKind::InconsistentHeadingHierarchy,
    }
}

impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckHeadings<'e> {

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
                self.push(max_depth_lint(settings, position));
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
                        consistency_lint = Some(inconsistent_hierarchy_lint(
                            position,
                            current_depth - depth - 1,
                        ));
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
