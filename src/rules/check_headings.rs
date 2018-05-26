use preamble::*;

rule_impl!(CheckHeadings, "Checks for erroneous headings."
=> examples:
    deep_heading,
    "===== deep heading =====\n",
    "Headings of depth more than three are not allowed. This a rule we use in \
     order to make the LaTeX export easier which only admits two heading \
     in an article. This heading has depth 5 and is thus not allowed.",
    "== normal heading ==\n",
    "This heading is of depth 2 and is thus allowed."
    => LintKind::MaxHeadingDepthViolation
;
    inconsistent_heading,
    "== top level ==\n\
     ==== low level ====\n",
    "The lower level heading has depth 4 and its parent has depth 2. Thus it \
     is two levels deeper than its parent which is not allowed.",
    "== top level ==\n\
     === low level ===\n",
    "The heading hierarchy is consistent. The lower level is exactly one level
     deeper than its parent heading."
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
            "Change your headings or your article structure to have a more \
             shallow structure.".into(),
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
        explanation_long:
            "If a heading has a higher heading than a previous heading, it is \
             considered a sub heading of this heading. Thus, headings make up \
             a hierarchy. But a heading more than one level deeper than its \
             parent makes no semantic sense. Heading levels should not be
             used to do text formatting!".into(),
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

        if let Element::Heading(ref heading) = *root {

            // is heading too deep?
            if heading.depth > settings.max_heading_depth {
                self.push(max_depth_lint(settings, &heading.position));
            }

            let mut consistency_lint = None;
            {
                // find parent heading
                let parent = self.path.iter().rev().find(
                    |e| if let Element::Heading(_) = ***e {**e != root} else {false}
                );

                if let Some(&&Element::Heading(ref parent)) = parent {
                    if heading.depth > parent.depth + 1 {
                        consistency_lint = Some(inconsistent_hierarchy_lint(
                            &heading.position,
                            heading.depth - parent.depth - 1,
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
