use preamble::*;
use template_spec::check_name;

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
;
    deprecated_formula_name,
    "{{formel|<math>bla</math>}}",
    "This template used to be called `formel` but has been renamed to `formula`.",
    "{{formula|<math>bla</math>}}",
    "The new naming conventions are used."
    => LintKind::DeprecatedTemplateName
;
    deprecated_arg_name,
    "{{formula|formel=<math>bla</math>}}",
    "Calling this template with a named argument is unecessarily verbose.",
    "{{formula|<math>bla</math>}}",
    "The template is called as conventions dictate."
    => LintKind::DeprecatedArgumentName
;
    missing_arg,
    "{{formula}}",
    "Calling the formula template without a math formula makes no sense.",
    "{{formula|<math>bla</math>}}",
    "The required argument \"1\" is given."
    => LintKind::MissingTemplateArgument
;
    illegal_argument_greeting,
    "{{formula|<math>bla</math>|greeting=This is just normal text.}}",
    "The formula template is called with wrong arguments, `greeting` does not belong to formula.",
    "{{formula|<math>bla</math>}}",
    "The first argument is a math formula."
    => LintKind::IllegalArgument
;
    illegal_formula_content,
    "{{formula|This is just normal text.}}",
    "A formula template must be given a math element.",
    "{{formula|<math>bla</math>}}",
    "The first argument is a math formula."
    => LintKind::IllegalArgumentContent
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

fn deprecated_name(
    position: &Span,
    used: &str,
    better: &str,
    objtext: &str,
    kind: LintKind,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("The {} name {} is deprecated!", objtext, used),
        explanation_long: format!(
            "For some {}s, the name they are referred to changes over time. \
             To make the transition easier, old and new names are allowed. \
             But eventually, only one name should be used.", objtext),
        solution: format!("Use {} instead of {}.", better, used),
        severity: Severity::Info,
        kind,
    }
}

fn missing_argument(
    position: &Span,
    name: &str,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("template argument {:?} is missing but required!", name),
        explanation_long: format!(
            "This template has arguments to tell it what to do. These can be named \
            (like {{name|argument_name=value}}) und anonymous {{name|value}}. \
            Anonymous arguments are equivalent to just enumerating named arguments: \
            ({{name|1=value}} <=> {{name|value}})"),
        solution: format!("Add a value for this argument."),
        severity: Severity::Error,
        kind: LintKind::MissingTemplateArgument,
    }
}

fn illegal_content(
    position: &Span,
    argument_name: &str,
    predicate_text: &str,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("The content of {} contains illegal elements!", argument_name),
        explanation_long: format!(
            "Some template arguments only allow certain kinds of text in their content. \
            In this case, the allowed values must fulfill the following property:\n{}",
            predicate_text),
        solution: format!("Take a look at the template specification or think about \
                           what makes sense."),
        severity: Severity::Error,
        kind: LintKind::IllegalArgumentContent,
    }
}

fn illegal_argument(
    position: &Span,
    argument_name: &str,
    template_name: &str,
    allowed: &[&str],
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("The argument {:?} is not allowed for {:?}",
            argument_name, template_name),
        explanation_long: format!(
            "{:?} only allows the following arguments:\n{:?}", template_name, allowed),
        solution: format!("Use one of the allowed template arguments."),
        severity: Severity::Warning,
        kind: LintKind::IllegalArgument,
    }
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

            let template_name = if let Some(text) = check_name(name) {
                text
            } else {
                self.push(invalid_template_name(position));
                return Ok(true)
            };

            let mut spec = None;
            for template in &settings.template_spec {
                if template.name == template_name {
                    spec = Some(template);
                    break;
                }
                if template.alternative_names.contains(&template_name.to_string()) {
                    self.push(deprecated_name(
                        position,
                        template_name,
                        &template.name,
                        "template",
                        LintKind::DeprecatedTemplateName
                    ));
                    spec = Some(template);
                    break;
                }
            }

            if let Some(spec) = spec {
                for arg_spec in &spec.attributes {

                    let mut exists = false;

                    for argument in content {
                        if let Element::TemplateArgument {
                            ref position,
                            ref name,
                            ref value
                        } = *argument {
                            if arg_spec.name == *name {
                                exists = true;
                            }
                            if arg_spec.alternative_names.contains(&name.to_string()) {
                                self.push(deprecated_name(
                                    position,
                                    name,
                                    &arg_spec.name,
                                    "argument",
                                    LintKind::DeprecatedArgumentName
                                ));
                                exists = true;
                            }

                            if !exists {
                                continue
                            }

                            if !(arg_spec.predicate)(value) {
                                self.push(illegal_content(
                                    position,
                                    name,
                                    &arg_spec.predicate_source
                                ));
                            }
                            break;
                        }
                    }
                    if !exists && arg_spec.priority == Priority::Required {
                        self.push(missing_argument(
                            position,
                            &arg_spec.name
                        ));
                    }
                }

                // find unspecified arguments
                let allowed_args: Vec<&str>
                    = spec.attributes.iter().map(|a| a.name.as_str()).collect();
                for argument in content {
                    if let Element::TemplateArgument { ref position, ref name, .. } = *argument {
                        let mut has_spec = false;
                        for arg_spec in &spec.attributes {
                            if arg_spec.name == *name {
                                has_spec = true;
                                break;
                            }
                        }

                        if !has_spec {
                            self.push(illegal_argument(
                                position,
                                name,
                                template_name,
                                allowed_args.as_slice()
                            ));
                        }
                    }
                }
            } else {
                self.push(template_not_allowed(position, template_name));
            }
        }
        Ok(true)
    }
}
