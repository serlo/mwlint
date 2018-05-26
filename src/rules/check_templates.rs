use preamble::*;
use mwparser_utils;

rule_impl!(CheckTemplates, "Checks for the correct use of templates."
=> examples:
    unknown_template,
    "{{unknown_template|arg1}}",
    "The template `unknown template` is not allowed or specified for this \
     project.",
    "{{Formel|<math>1+1=2</math>}}",
    "The {{Formel|...}} template exists and is used properly."
    => LintKind::TemplateNotAllowed
;
    formatted_template_name,
    "{{template-{{foo}}|arg}}",
    "This template's name contains a template, which can lead to problems \
     with other tools.",
    "{{Formel|<math>x^2</math>}}",
    "To ensure compatibility, only use alphanumeric characters plus _,.,: and \
     white spaces."
    => LintKind::InvalidTemplateName
;
    deprecated_template_name,
    "{{Hinweis|Important remark}}",
    "This template used to be called `Hinweis`. However we use the template \
     {{:Mathe für Nicht-Freaks: Vorlage:Hinweis|...}} in this project since \
     we have a personalized formating for it.",
    "{{:Mathe für Nicht-Freaks: Vorlage:Hinweis|Important remark}}",
    "Our naming conventions are used."
    => LintKind::DeprecatedTemplateName
// Currently we have no deprecated arguments. Uncomment this when there some.
//;
//    deprecated_arg_name,
//    "{{Formel|formel=<math>x^2</math>}}",
//    "Calling this template with a named argument is unecessarily verbose.",
//    "{{Formel|<math>x^2</math>}}",
//    "The template is called with unnamed parameters."
//    => LintKind::DeprecatedArgumentName
;
    missing_arg,
    "{{Formel}}",
    "The {{Formel|...}} template needs a parameter for the formula.",
    "{{Formel|<math>x^2</math>}}",
    "The required unnamed argument \"1\" with the formula is given."
    => LintKind::MissingTemplateArgument
;
    illegal_argument_greeting,
    "{{Formel|<math>x^2</math>|greeting=This is just normal text.}}",
    "The formula template is called with wrong arguments, `greeting` is no \
     valid parameter for the formula template.",
    "{{Formel|<math>x^2</math>}}",
    "The invalid parameter was deleted."
    => LintKind::IllegalArgument
;
    illegal_formula_content,
    "{{Formel|<math>x^2</math> and <math>b^2</math>}}",
    "A formula template must be given only a math element.",
    "{{Formel|<math>x^2 \\text{ and } b^2</math>}}",
    "The formula template only contains a math element."
    => LintKind::IllegalArgumentContent
);

fn template_not_allowed(
    position: &Span,
    name: &str,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("{:?}-templates are not allowed / specified!", name),
        explanation_long:
            "Only a specific set of templates is allowed for this project. \
             This rule is in place to make sure elements with the same meaning \
             Are recognized as such and formatted in the same way.".into(),
        solution: "Use another template. Maybe this is just a spelling mistake?".into(),
        severity: Severity::Error,
        kind: LintKind::TemplateNotAllowed,
    }
}

fn invalid_template_name(
    position: &Span,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: "formatted text is not allowed in template names!".into(),
        explanation_long:
            "Using text markup or even block elements in template names may cause \
            unexpected behaviour and incompatibility with external utilities. \
            Good template names are expressive, easy to type and consist of only \
            alphanumerical characters plus _,.,: .".into(),
        solution: "Use better template names.".into(),
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
        explanation_long:
            "This template has arguments to tell it what to do. These can be named \
            (like {{name|argument_name=value}}) und anonymous {{name|value}}. \
            Anonymous arguments are equivalent to just enumerating named arguments: \
            ({{name|1=value}} <=> {{name|value}})".into(),
        solution: "Add a value for this argument.".into(),
        severity: Severity::Error,
        kind: LintKind::MissingTemplateArgument,
    }
}

fn illegal_content(
    position: &Span,
    argument_name: &str,
    reason: &str,
    predicate_text: &str,
) -> Lint {
    Lint {
        position: position.clone(),
        explanation: format!("This markup is not allowed in the content of {}: {}",
            argument_name, reason),
        explanation_long: format!(
            "Some template arguments only allow certain kinds of text in their content. \
            In this case, the allowed values must fulfill the following property:\n{}",
            predicate_text),
        solution: "Take a look at the template specification or think about \
                   what makes sense.".into(),
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
        solution: "Use one of the allowed template arguments.".into(),
        severity: Severity::Warning,
        kind: LintKind::IllegalArgument,
    }
}

impl<'e, 's> Traversion<'e, &'s Settings<'s>> for CheckTemplates<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            _: &Settings,
            _: &mut io::Write) -> io::Result<bool> {

        if let Element::Template(ref template) = *root {

            if mwparser_utils::is_plain_text(&template.name).is_err() {
                self.push(invalid_template_name(&template.position));
            }

            let template_name = extract_plain_text(&template.name).trim().to_lowercase();

            if template_name.starts_with("#lst:") {
                return Ok(true)
            }

            if let Some(template_spec) = mwparser_utils::spec_of(&template_name) {

                if mwparser_utils::parse_template(&template).is_none() {
                    for arg_spec in &template_spec.attributes {
                        let exists = find_arg(&template.content, &arg_spec.names).is_some();
                        if !exists && arg_spec.priority == mwparser_utils::spec_meta::Priority::Required {
                            self.push(missing_argument(
                                &template.position,
                                &arg_spec.default_name().trim().to_lowercase(),
                            ));
                        }
                    }
                    return Ok(true);
                };

                let default_name = template_spec.default_name().trim().to_lowercase();
                if template_name != default_name {
                     self.push(deprecated_name(
                        &template.position,
                        &template_name,
                        &default_name,
                        "template",
                        LintKind::DeprecatedTemplateName
                    ));
                }

                for arg_spec in &template_spec.attributes {

                    let default_argname = arg_spec.default_name()
                        .trim().to_lowercase();

                    if let Some(&Element::TemplateArgument(ref arg))
                        = find_arg(&template.content, &arg_spec.names)
                    {
                        let actual_name = &arg.name;
                        if actual_name != &default_argname {
                            self.push(deprecated_name(
                                &arg.position,
                                actual_name,
                                &default_argname,
                                "argument",
                                LintKind::DeprecatedArgumentName
                            ));
                        }

                        if let Err(error) = (arg_spec.predicate)(&arg.value) {
                            self.push(illegal_content(
                                error.tree.map(|e| e.get_position()).unwrap_or(&arg.position),
                                actual_name,
                                &error.cause,
                                &arg_spec.predicate_name
                            ));
                        }
                    }
                }

                // find unspecified arguments
                let allowed_args: Vec<&str>
                    = template_spec.attributes.iter()
                        .map(|a| a.default_name()).collect();

                for argument in &template.content {
                    if let Element::TemplateArgument(ref arg) = *argument {
                        let name = arg.name.trim().to_lowercase();
                        let has_spec = template_spec.attributes.iter()
                            .any(|arg_spec| arg_spec.names.contains(&name));

                        if !has_spec {
                            self.push(illegal_argument(
                                &arg.position,
                                &name,
                                &template_name,
                                allowed_args.as_slice()
                            ));
                        }
                    }
                }
            } else {
                self.push(template_not_allowed(&template.position, &template_name));
            }
        }
        Ok(true)
    }
}
