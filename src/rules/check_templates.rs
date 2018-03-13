use preamble::*;

rule_impl!(CheckTemplates, "Checks for the correct use of templates."
=> examples:
    unknown_template,
    "{{unknown_template|arg1}}",
    "The template `unknown template` is not allowed / specified for this project.",
    "{{formula|<math>1+1=2</math>}}",
    "The specification for this template exists in this project and allows for its use."
    => LintKind::TemplateNotAllowed
;
    formatted_template_name,
    "{{this is {{bla}} text|arg}}",
    "This template's name contains a template, \
     which can lead to problems with other tools.",
    "{{formula|<math>bla</math>}}",
    "To ensure compatibility, only use alphanumeric characters plus _,.,:"
    => LintKind::InvalidTemplateName
);

fn template_not_allowed(
    position: &Span,
    name: &str,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("{:?}-templates are not allowed / specified!", name),
        explanation_long: format!(
            "Only a specific set of templates is allowed for this project. \
             This rule is in place to make sure elements with the same meaning \
             Are recognized as such and formatted in the same way."),
        solution: format!("Use another template. Maybe this is just a spelling mistake?"),
        severity: Severity::Error,
        kind: LintKind::TemplateNotAllowed,
    }
}

fn invalid_template_name(
    position: &Span,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("formatted text is not allowed in template names!"),
        explanation_long: format!(
            "Using text markup or even block elements in template names may cause \
            unexpected behaviour and incompatibility with external utilities. \
            Good template names are expressive, easy to type and consist of only \
            alphanumerical characters plus _,.,: ."),
        solution: format!("Use better template names."),
        severity: Severity::Error,
        kind: LintKind::InvalidTemplateName,
    }
}

fn check_name(name: &[Element]) -> Option<&str> {
    if name.len() != 1 {
        return None
    }
    match name.first() {
        Some(&Element::Text { ref text, .. }) => return Some(text),
        Some(&Element::Paragraph { ref content, .. }) => {
            if content.len() != 1 {
                return None
            }
            if let Some(&Element::Text { ref text, .. }) = content.first() {
                return Some(text)
            }
        },
        _ => (),
    };
    None
}

impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckTemplates<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            settings: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::Template {
            ref position,
            ref name,
            ref content
        } = *root {

            let name = if let Some(text) = check_name(name) {
                text
            } else {
                self.push(invalid_template_name(position));
                return Ok(true)
            };

            let mut spec = None;
            for template in &settings.template_spec {
                if template.name == name {
                    spec = Some(template);
                    break;
                }
            }

            if let Some(spec) = spec {

            } else {
                self.push(template_not_allowed(position, name));
            }
        }
        Ok(true)
    }
}
