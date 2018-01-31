use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;


/// Verify all templates used are whitelisted.
#[derive(Debug, Default)]
pub struct CheckTemplateWhitelist<'e> {
    pub path: Vec<&'e Element>,
    pub lints: Vec<Lint>,
}

rule_impl!("check template whitelist",
           "only whitelisted templates are allowed.", CheckTemplateWhitelist);

impl<'e, 's> Traversion<'e, &'s RuleParameters> for CheckTemplateWhitelist<'e> {

    path_impl!();

    fn work(&mut self,
            root: &'e Element,
            parameters: &RuleParameters,
            _: &mut io::Write) -> io::Result<bool> {


        if let Element::Template { ref name, ref position, .. } = *root {

            let template_name_err = Lint {
                position: position.clone(),
                message: "Template name is not text-only. This could make further \
                          processing more complicated.".into(),
                solution: "Maybe a flat template name is also suitable here.".into(),
                severity: Severity::Info,
            };

            if name.len() != 1 {
                self.push(template_name_err);
            } else {
               match name.first() {
                    Some(&Element::Text { ref text, ref position }) => {
                        self.check_text(text.trim(), position, parameters);
                    },
                    Some(&Element::Paragraph { ref content, .. }) => {
                        if content.len() != 1 {
                            self.push(template_name_err);
                        } else {
                            if let Some(&Element::Text { ref text, ref position })
                                = content.first() {
                                    self.check_text(text.trim(), position, parameters);
                            } else {
                                self.push(template_name_err);
                            }
                        }
                    },
                    _ => {
                        self.push(template_name_err);
                    }
                }
            }
        };
        Ok(true)
    }
}

impl<'e> CheckTemplateWhitelist<'e> {
    fn check_text(&mut self, text: &str, position: &Span, params: &RuleParameters) {
        if !params.template_whitelist.contains(&text.into()) {

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
            self.push(err);
        }
    }
}

